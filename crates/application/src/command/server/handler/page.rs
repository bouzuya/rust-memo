use std::str::FromStr;

use super::helpers::is_all;
use crate::template::{PageItemTemplate, PageTemplate, PageWithTitle};
use actix_web::{web, HttpResponse, ResponseError};
use askama::Template;
use entity::{PageId, PagePath, TitlePath};
use thiserror::Error;
use use_case::{HasPageRepository, PageRepository};

// TODO:
#[derive(Debug, Error)]
#[error("error")]
struct MyError(String);

impl ResponseError for MyError {}

pub async fn page<T: HasPageRepository>(
    req: actix_web::HttpRequest,
    data: web::Data<T>,
) -> actix_web::Result<HttpResponse> {
    let app = data.get_ref();
    let all = is_all(&req);
    let params: (String,) = req.match_info().load()?;
    let page_id = PageId::from_str(&params.0)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "invalid page_id format"))?;
    let title = app
        .page_repository()
        .find_content(&page_id)
        .map_err(|_| MyError(format!("IO Error: {}", page_id)))?
        .map(|page_content| page_content.title())
        .ok_or_else(|| MyError(format!("page_id not found: {}", page_id)))?;
    let page_graph = app
        .page_repository()
        .load_page_graph()
        .map_err(|_| actix_web::Error::from(()))?;
    let linked_by = page_graph
        .find_ids_link_to(&title)
        .iter()
        .filter(|page_id| all || !page_graph.is_obsoleted(page_id))
        .map(|page_id| {
            let title = app
                .page_repository()
                .find_content(page_id)
                .map_err(|_| MyError(format!("IO Error: {}", page_id)))?
                .map(|page_content| page_content.title())
                .ok_or_else(|| MyError(format!("page_id not found: {}", page_id)))?;
            Ok(PageWithTitle {
                id: page_id.to_string(),
                obsoleted: page_graph.is_obsoleted(page_id),
                title: title.to_string(),
                url: PagePath::from(*page_id).to_string(),
            })
        })
        .collect::<actix_web::Result<Vec<PageWithTitle>>>()?;
    let obsoleted_by = page_graph
        .obsoleted_by(&page_id)
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            obsoleted: page_graph.is_obsoleted(page_id),
            url: PagePath::from(*page_id).to_string(),
        })
        .collect::<Vec<PageItemTemplate>>();
    let md = app
        .page_repository()
        .find_content(&page_id)
        .map_err(|_| MyError(format!("IO error: {}", page_id)))?
        .map(|mut page_content| {
            page_content.ensure_links();
            page_content
        })
        .map(String::from)
        .ok_or_else(|| MyError(format!("file not found: {}", page_id)))?;
    let parser = pulldown_cmark::Parser::new(&md);
    let mut markdown_html = String::new();
    pulldown_cmark::html::push_html(&mut markdown_html, parser);
    let template = PageTemplate {
        linked_by: &linked_by,
        page_id: &page_id.to_string(),
        page_url: &PagePath::from(page_id).to_string(),
        title: title.as_str(),
        title_url: &TitlePath::from(title.clone()).to_string(),
        html: markdown_html,
        obsoleted_by: &obsoleted_by,
    };
    let html = template.render().map_err(|_| actix_web::Error::from(()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
