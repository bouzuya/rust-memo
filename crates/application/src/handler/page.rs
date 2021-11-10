use std::str::FromStr;

use crate::handler_helpers::is_all;
use crate::helpers::{is_obsoleted, read_linked_map, read_obsoleted_map, read_title, to_file_name};
use crate::template::{PageItemTemplate, PageTemplate, PageWithTitle};
use crate::url_helpers::{page_url, title_url};
use actix_web::HttpResponse;
use askama::Template;
use entity::PageId;

pub async fn page(req: actix_web::HttpRequest) -> std::io::Result<HttpResponse> {
    let all = is_all(&req);
    let params: (String,) = req.match_info().load().unwrap();
    let page_id = PageId::from_str(&params.0)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "invalid page_id format"))?;
    let title = read_title(&page_id);
    let linked_map = read_linked_map()?;
    let obsoleted_map = read_obsoleted_map()?;
    let linked_by = linked_map
        .get(&title)
        .unwrap_or(&std::collections::BTreeSet::new())
        .iter()
        .map(|page_id| PageWithTitle {
            id: page_id.to_string(),
            obsoleted: is_obsoleted(&obsoleted_map, &page_id),
            title: read_title(&page_id).to_string(),
            url: page_url(&page_id),
        })
        .filter(|template| all || !template.obsoleted)
        .collect::<Vec<PageWithTitle>>();
    let obsoleted_by = obsoleted_map
        .get(&page_id)
        .unwrap_or(&std::collections::BTreeSet::new())
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            obsoleted: is_obsoleted(&obsoleted_map, &page_id),
            url: page_url(&page_id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let page_file_name = to_file_name(&page_id);
    let md = std::fs::read_to_string(page_file_name)?;
    let parser = pulldown_cmark::Parser::new(&md);
    let mut markdown_html = String::new();
    pulldown_cmark::html::push_html(&mut markdown_html, parser);
    let template = PageTemplate {
        html: markdown_html,
        linked_by: &linked_by,
        page_id: &page_id.to_string(),
        page_url: &page_url(&page_id),
        title: title.as_str(),
        title_url: &title_url(&title),
        obsoleted_by: &obsoleted_by,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
