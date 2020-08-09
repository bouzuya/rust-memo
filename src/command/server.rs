use crate::helpers::{
    list_ids, read_obsoleted_map, read_title_map, to_file_name, to_page_url, to_title_url,
};
use crate::page_id::PageId;
use actix_web::{web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "pages.html")]
struct PagesTemplate<'a> {
    title: &'a str,
    pages: &'a [PageItemTemplate],
}

struct PageItemTemplate {
    id: String,
    url: String,
}

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate<'a> {
    title: &'a str,
    html: String,
    obsoleted_by: &'a [PageItemTemplate],
}

#[derive(Template)]
#[template(path = "titles.html")]
struct TitlesTemplate<'a> {
    title: &'a str,
    titles: &'a [TitlesItemTemplate],
}

struct TitlesItemTemplate {
    title: String,
    url: String,
}

#[derive(Template)]
#[template(path = "title.html")]
struct TitleTemplate<'a> {
    title: &'a str,
    pages: &'a [PageItemTemplate],
}

async fn index() -> impl actix_web::Responder {
    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, format!("/pages"))
        .finish()
}

async fn pages() -> std::io::Result<HttpResponse> {
    let page_ids = list_ids()?;
    let pages = page_ids
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            url: to_page_url(&page_id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let template = PagesTemplate {
        title: "/pages",
        pages: &pages,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn page(params: web::Path<(String,)>) -> std::io::Result<HttpResponse> {
    let page_id = PageId::from_str(&params.0).ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "invalid page_id format",
    ))?;
    let obsoleted_map = read_obsoleted_map()?;
    let obsoleted_by = obsoleted_map
        .get(&page_id)
        .unwrap_or(&std::collections::BTreeSet::new())
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            url: to_page_url(&page_id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let page_file_name = to_file_name(&page_id);
    let md = std::fs::read_to_string(page_file_name)?;
    let parser = pulldown_cmark::Parser::new(&md);
    let mut markdown_html = String::new();
    pulldown_cmark::html::push_html(&mut markdown_html, parser);
    let template = PageTemplate {
        title: &to_page_url(&page_id),
        html: markdown_html,
        obsoleted_by: &obsoleted_by,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn titles() -> std::io::Result<HttpResponse> {
    let title_map = read_title_map()?;
    let titles = title_map
        .iter()
        .map(|(title, _)| TitlesItemTemplate {
            title: title.to_owned(),
            url: to_title_url(&title),
        })
        .collect::<Vec<TitlesItemTemplate>>();
    let template = TitlesTemplate {
        title: "/titles",
        titles: &titles,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn title(params: web::Path<(String,)>) -> std::io::Result<HttpResponse> {
    let title_map = read_title_map()?;
    let title = &params.0;
    if let Some(page_ids) = title_map.get(title) {
        let pages = page_ids
            .iter()
            .map(|page_id| PageItemTemplate {
                id: page_id.to_string(),
                url: to_page_url(&page_id),
            })
            .collect::<Vec<PageItemTemplate>>();
        let template = TitleTemplate {
            title: &to_title_url(title),
            pages: &pages,
        };
        let html = template.render().unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[actix_rt::main]
async fn run_server() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(index))
            .route("/pages", web::get().to(pages))
            .route("/pages/{id}", web::get().to(page))
            .route("/titles", web::get().to(titles))
            .route("/titles/{title}", web::get().to(title))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

pub fn server() -> Result<(), Box<dyn std::error::Error>> {
    Ok(run_server()?)
}
