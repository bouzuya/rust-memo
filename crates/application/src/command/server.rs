use crate::handler::index::index;
use crate::handler::page::page;
use crate::handler::pages::pages;
use crate::handler::title::title;
use crate::handler::titles::titles;
use actix_web::web;
use use_case::HasPageRepository;

pub async fn server<T: HasPageRepository + Send + Sync + 'static>(app: T) -> std::io::Result<()> {
    let data = web::Data::new(app);
    let mut listenfd = listenfd::ListenFd::from_env();
    let mut server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/pages", web::get().to(pages))
            .route("/pages/{id}", web::get().to(page::<T>))
            .route("/titles", web::get().to(titles))
            .route("/titles/{title}", web::get().to(title))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };
    println!("listening: ");
    for (addr, scheme) in server.addrs_with_scheme().iter() {
        println!("- {}://{}", scheme, addr);
    }
    server.run().await
}
