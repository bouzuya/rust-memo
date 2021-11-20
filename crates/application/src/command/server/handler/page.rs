use std::str::FromStr;

use crate::handler_helpers::is_all;
use crate::helpers::{is_obsoleted, read_linked_map, read_obsoleted_map};
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
) -> actix_web::error::Result<HttpResponse> {
    let app = data.get_ref();
    let all = is_all(&req);
    let params: (String,) = req.match_info().load().unwrap();
    let page_id = PageId::from_str(&params.0)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "invalid page_id format"))?;
    let title = app
        .page_repository()
        .find_title(&page_id)
        .map_err(|_| MyError(format!("IO Error: {}", page_id)))?
        .ok_or_else(|| MyError(format!("page_id not found: {}", page_id)))?;
    let linked_map = read_linked_map()?;
    let obsoleted_map = read_obsoleted_map()?;
    let linked_by = linked_map
        .get(&title)
        .unwrap_or(&std::collections::BTreeSet::new())
        .iter()
        .filter(|&page_id| all || !is_obsoleted(&obsoleted_map, page_id))
        .map(|page_id| {
            let title = app
                .page_repository()
                .find_title(page_id)
                .map_err(|_| MyError(format!("IO Error: {}", page_id)))?
                .ok_or_else(|| MyError(format!("page_id not found: {}", page_id)))?;
            Ok(PageWithTitle {
                id: page_id.to_string(),
                obsoleted: is_obsoleted(&obsoleted_map, page_id),
                title: title.to_string(),
                url: PagePath::from(*page_id).to_string(),
            })
        })
        .collect::<actix_web::error::Result<Vec<PageWithTitle>>>()?;
    let obsoleted_by = obsoleted_map
        .get(&page_id)
        .unwrap_or(&std::collections::BTreeSet::new())
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            obsoleted: is_obsoleted(&obsoleted_map, page_id),
            url: PagePath::from(*page_id).to_string(),
        })
        .collect::<Vec<PageItemTemplate>>();
    let md = app
        .page_repository()
        .find_content(&page_id)
        .map_err(|_| MyError(format!("IO error: {}", page_id)))?
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
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
