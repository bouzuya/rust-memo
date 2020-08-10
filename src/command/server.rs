use crate::helpers::{list_ids, read_obsoleted_map, read_title_map, to_file_name};
use crate::page_id::PageId;
use crate::page_title::PageTitle;
use crate::url_helpers::{page_url, pages_url, title_url, titles_url};
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
    obsoleted: bool,
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
        .header(actix_web::http::header::LOCATION, pages_url())
        .finish()
}

async fn pages() -> std::io::Result<HttpResponse> {
    let obsoleted_map = read_obsoleted_map()?;
    let page_ids = list_ids()?;
    let pages = page_ids
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            obsoleted: obsoleted_map.get(&page_id).is_some(),
            url: page_url(&page_id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let template = PagesTemplate {
        title: &pages_url(),
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
            obsoleted: obsoleted_map.get(&page_id).is_some(),
            url: page_url(&page_id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let page_file_name = to_file_name(&page_id);
    let md = std::fs::read_to_string(page_file_name)?;
    let parser = pulldown_cmark::Parser::new(&md);
    let mut markdown_html = String::new();
    pulldown_cmark::html::push_html(&mut markdown_html, parser);
    let template = PageTemplate {
        title: &page_url(&page_id),
        html: markdown_html,
        obsoleted_by: &obsoleted_by,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn titles(req: actix_web::HttpRequest) -> std::io::Result<HttpResponse> {
    use std::str::FromStr;
    let all = match url::Url::from_str(&format!("http://example.com{}", req.uri().to_string())) {
        Err(_) => false,
        Ok(url) => {
            let map = url
                .query_pairs()
                .into_owned()
                .collect::<std::collections::HashMap<String, String>>();
            map.get("all") == Some(&"true".to_owned())
        }
    };
    let obsoleted_map = read_obsoleted_map()?;
    let title_map = read_title_map()?;
    let titles = title_map
        .iter()
        .filter(|(_, page_ids)| {
            all || page_ids
                .iter()
                .any(|page_id| obsoleted_map.get(page_id).is_none())
        })
        .map(|(title, _)| TitlesItemTemplate {
            title: title.to_string(),
            url: title_url(&title),
        })
        .collect::<Vec<TitlesItemTemplate>>();
    let template = TitlesTemplate {
        title: &titles_url(),
        titles: &titles,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn title(params: web::Path<(String,)>) -> std::io::Result<HttpResponse> {
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
            .collect::<Vec<PageItemTemplate>>();
        let template = TitleTemplate {
            title: &title_url(&title),
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
    server.run().await
}
