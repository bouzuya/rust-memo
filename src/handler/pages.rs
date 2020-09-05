use crate::handler_helpers::is_all;
use crate::helpers::{is_obsoleted, list_ids, read_obsoleted_map};
use crate::template::{PageItemTemplate, PagesTemplate};
use crate::url_helpers::page_url;
use crate::url_helpers::pages_url;
use actix_web::HttpResponse;
use askama::Template;

pub async fn pages(req: actix_web::HttpRequest) -> std::io::Result<HttpResponse> {
  let all = is_all(&req);
  let obsoleted_map = read_obsoleted_map()?;
  let page_ids = list_ids()?;
  let pages = page_ids
    .iter()
    .map(|page_id| PageItemTemplate {
      id: page_id.to_string(),
      obsoleted: is_obsoleted(&obsoleted_map, &page_id),
      url: page_url(&page_id),
    })
    .filter(|template| all || !template.obsoleted)
    .collect::<Vec<PageItemTemplate>>();
  let template = PagesTemplate {
    title: &pages_url(),
    pages: &pages,
  };
  let html = template.render().unwrap();
  Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
