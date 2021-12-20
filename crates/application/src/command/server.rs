mod handler;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use self::handler::{index, page, page_create, page_update, pages, title, title_pages, titles};
use actix_web::web;
use anyhow::Context as _;
use entity::{Page, PageContent, PageId};
use use_case::{HasListPagesUseCase, HasListTitlesUseCase, HasPageRepository, PageRepository};
use watchexec::{
    config::{Config, ConfigBuilder},
    error::Result,
    pathop::PathOp,
    run::{watch, ExecHandler, Handler},
};

pub async fn server<
    T: HasListTitlesUseCase + HasListPagesUseCase + HasPageRepository + Send + Sync + 'static,
>(
    app: T,
) -> anyhow::Result<()> {
    let page_repository = app.page_repository();
    for page_id in page_repository.find_ids()? {
        if let Some(page) = page_repository.find_by_id(&page_id)? {
            page_repository.save_cache(page)?;
        }
    }

    let app = Arc::new(Mutex::new(app));

    // run file watcher
    let app1 = app.clone();
    thread::spawn(move || -> anyhow::Result<()> {
        let config = ConfigBuilder::default()
            .paths(vec![".".into()])
            .cmd(vec![":".into()])
            .build()
            .context("fail")?;
        let handler = MyHandler::<T>(ExecHandler::new(config)?, app1);
        watch(&handler).context("fail")?;
        Ok(())
    });

    // run http server
    let data = web::Data::new(app);
    let mut listenfd = listenfd::ListenFd::from_env();
    let mut server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/pages", web::get().to(pages::<T>))
            .route("/pages", web::post().to(page_create::<T>))
            .route("/pages/{id}", web::get().to(page::<T>))
            .route("/pages/{id}", web::patch().to(page_update::<T>))
            .route("/titles", web::get().to(titles::<T>))
            .route("/titles/{title}", web::get().to(title::<T>))
            .route("/titles/{title}/pages", web::get().to(title_pages::<T>))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };
    println!("listening: ");
    for (addr, scheme) in server.addrs_with_scheme().iter() {
        println!("- {}://{}", scheme, addr);
    }
    Ok(server.run().await?)
}

struct MyHandler<T>(ExecHandler, Arc<Mutex<T>>);

impl<T: HasPageRepository + Send + Sync + 'static> Handler for MyHandler<T> {
    fn on_manual(&self) -> Result<bool> {
        self.0.on_manual()
    }

    fn on_update(&self, ops: &[PathOp]) -> Result<bool> {
        for op in ops {
            match op.op {
                None => continue,
                Some(o) => {
                    let page_id = if let Some(s) = op.path.as_os_str().to_str() {
                        match PageId::from_like_str(s) {
                            Err(_) => continue,
                            Ok(page_id) => page_id,
                        }
                    } else {
                        continue;
                    };

                    let app = self.1.lock().map_err(|_| {
                        watchexec::error::Error::Generic("failed to lock".to_string())
                    })?;

                    if PathOp::is_rename(o) {
                        if !op.path.exists() {
                            println!("rename: {:?} (from)", page_id);
                            app.page_repository().destroy_cache(&page_id).map_err(|_| {
                                watchexec::error::Error::Generic("failed to destroy".to_string())
                            })?;
                        } else {
                            println!("rename: {:?} (to)", page_id);
                            let found = app
                                .page_repository()
                                .find_by_id(&page_id)
                                .map_err(|_| {
                                    watchexec::error::Error::Generic(
                                        "failed to find by id".to_string(),
                                    )
                                })?
                                .ok_or_else(|| {
                                    watchexec::error::Error::Generic(
                                        "failed to find by id".to_string(),
                                    )
                                })?;
                            app.page_repository().save_cache(found).map_err(|_| {
                                watchexec::error::Error::Generic("failed to save".to_string())
                            })?;
                        }
                    } else {
                        if PathOp::is_create(o) {
                            println!("create: {:?}", page_id);
                            app.page_repository()
                                .save_cache(Page::new(page_id, PageContent::from("".to_string())))
                                .map_err(|_| {
                                    watchexec::error::Error::Generic("failed to save".to_string())
                                })?;
                        }
                        if PathOp::is_remove(o) {
                            println!("remove: {:?}", page_id);
                            app.page_repository().destroy_cache(&page_id).map_err(|_| {
                                watchexec::error::Error::Generic("failed to remove".to_string())
                            })?;
                        }
                        if PathOp::is_write(o) {
                            println!("on write: {:?}", page_id);
                            let found = app
                                .page_repository()
                                .find_by_id(&page_id)
                                .map_err(|_| {
                                    watchexec::error::Error::Generic(
                                        "failed to find by id".to_string(),
                                    )
                                })?
                                .ok_or_else(|| {
                                    watchexec::error::Error::Generic(
                                        "failed to find by id".to_string(),
                                    )
                                })?;
                            app.page_repository().save_cache(found).map_err(|_| {
                                watchexec::error::Error::Generic("failed to save".to_string())
                            })?;
                        }
                        if PathOp::is_meta(o) {
                            println!("on meta: {:?}", page_id);
                        }
                    }

                    println!("on update {:?} {:?}", o, page_id);
                }
            }
        }
        self.0.on_update(ops)
    }

    fn args(&self) -> Config {
        self.0.args()
    }
}
