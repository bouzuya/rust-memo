use crate::handler::index::index;
use crate::handler::page::page;
use crate::handler::pages::pages;
use crate::handler_helpers::is_all;
use crate::helpers::{is_obsoleted, read_obsoleted_map, read_title_map};
use crate::page_title::PageTitle;
use crate::template::{PageItemTemplate, TitleTemplate, TitlesItemTemplate, TitlesTemplate};
use crate::url_helpers::{page_url, title_url, titles_url};
use actix_web::{web, HttpResponse};
use askama::Template;

async fn titles(req: actix_web::HttpRequest) -> std::io::Result<HttpResponse> {
    let all = is_all(&req);
    let obsoleted_map = read_obsoleted_map()?;
    let title_map = read_title_map()?;
    let titles = title_map
        .iter()
        .map(|(title, page_ids)| TitlesItemTemplate {
            obsoleted: !page_ids
                .iter()
                .any(|page_id| !is_obsoleted(&obsoleted_map, page_id)),
            title: title.to_string(),
            url: title_url(&title),
        })
        .filter(|template| all || !template.obsoleted)
        .collect::<Vec<TitlesItemTemplate>>();
    let template = TitlesTemplate {
        show_all: all,
        title: &titles_url(),
        titles: &titles,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn title(req: actix_web::HttpRequest) -> std::io::Result<HttpResponse> {
    let all = is_all(&req);
    let params: (String,) = req.match_info().load().unwrap();
    let obsoleted_map = read_obsoleted_map()?;
    let title_map = read_title_map()?;
    let title = PageTitle::from_str(&params.0);
    if let Some(page_ids) = title_map.get(&title) {
        let pages = page_ids
            .iter()
            .map(|page_id| PageItemTemplate {
                id: page_id.to_string(),
                obsoleted: obsoleted_map.get(page_id).is_some(),
                url: page_url(&page_id),
            })
            .filter(|template| all || !template.obsoleted)
            .collect::<Vec<PageItemTemplate>>();
        let template = TitleTemplate {
            title: title.as_str(),
            title_url: &title_url(&title),
            pages: &pages,
        };
        let html = template.render().unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[actix_rt::main]
pub async fn server() -> std::io::Result<()> {
    let mut listenfd = listenfd::ListenFd::from_env();
    let mut server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(index))
            .route("/pages", web::get().to(pages))
            .route("/pages/{id}", web::get().to(page))
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
