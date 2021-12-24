use std::sync::{Arc, Mutex};

use super::helpers::is_all;
use crate::template::{PageItemTemplate, PagesTemplate};
use crate::url_helpers::pages_url;
use actix_web::{web::Data, HttpResponse};
use askama::Template;
use entity::PagePath;
use use_case::{HasListPagesUseCase, ListPagesUseCase};

pub async fn page_list<T: HasListPagesUseCase>(
    req: actix_web::HttpRequest,
    data: Data<Arc<Mutex<T>>>,
) -> actix_web::Result<HttpResponse> {
    let app = data
        .get_ref()
        .lock()
        .map_err(|_| actix_web::Error::from(()))?;
    let all = is_all(&req);
    let pages = app
        .list_pages_use_case()
        .list_pages(all)
        .map_err(|_| actix_web::Error::from(()))?;
    let pages = pages
        .into_iter()
        .map(|page| PageItemTemplate {
            id: page.0.to_string(),
            obsoleted: page.1,
            url: PagePath::from(page.0).to_string(),
        })
        .collect::<Vec<PageItemTemplate>>();
    let template = PagesTemplate {
        title: &pages_url(),
        pages: &pages,
    };
    let html = template.render().map_err(|_| actix_web::Error::from(()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
