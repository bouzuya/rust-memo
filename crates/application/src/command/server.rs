mod handler;

use std::thread;

use self::handler::{index, page, pages, title, title_pages, titles};
use actix_web::web;
use anyhow::Context as _;
use entity::PageId;
use use_case::{HasListPagesUseCase, HasListTitlesUseCase, HasPageRepository};
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
    // run file watcher
    thread::spawn(|| -> anyhow::Result<()> {
        let config = ConfigBuilder::default()
            .paths(vec![".".into()])
            .cmd(vec![":".into()])
            .build()
            .context("fail")?;
        let handler = MyHandler(ExecHandler::new(config)?);
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
            .route("/pages/{id}", web::get().to(page::<T>))
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

struct MyHandler(ExecHandler);

impl Handler for MyHandler {
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

                    println!("on update {:?} {:?}", o, page_id);
                }
            }
        }
        // TODO: CREATE => add_page
        // TODO: WRITE => remove_page -> add_page
        // TODO: CHMOD => do nothing
        // TODO: REMOVE => remove_page
        // TODO: RENAME => remove_page -> add_page
        // TODO: CLOSE_WRITE ...
        // TODO: RESCAN      ...
        println!("on update {:?}", ops);
        self.0.on_update(ops)
    }

    fn args(&self) -> Config {
        self.0.args()
    }
}
