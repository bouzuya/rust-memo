mod handler;

use self::handler::{index, page, pages, title, title_pages, titles};
use actix_web::web;
use use_case::{HasListTitlesUseCase, HasPageRepository};

pub async fn server<T: HasListTitlesUseCase + HasPageRepository + Send + Sync + 'static>(
    app: T,
) -> anyhow::Result<()> {
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
