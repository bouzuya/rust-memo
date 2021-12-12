use std::sync::{Arc, Mutex};

use super::helpers::is_all;
use crate::template::{PageItemTemplate, TitleNotFoundTemplate, TitleTemplate};
use actix_web::{web, HttpResponse};
use askama::Template;
use entity::{PageId, PagePath, PageTitle, TitlePath};
use use_case::{HasPageRepository, PageRepository};

pub async fn title<T: HasPageRepository>(
    req: actix_web::HttpRequest,
    data: web::Data<Arc<Mutex<T>>>,
) -> actix_web::Result<HttpResponse> {
    let app = data
        .get_ref()
        .lock()
        .map_err(|_| actix_web::Error::from(()))?;
    let all = is_all(&req);
    let params: (String,) = req.match_info().load()?;
    let page_graph = app
        .page_repository()
        .load_page_graph()
        .map_err(|_| actix_web::Error::from(()))?;
    let title = PageTitle::from(params.0);
    let page_ids = page_graph.titled(&title);
    if page_ids.is_empty() {
        let template = TitleNotFoundTemplate {
            title: title.as_str(),
            title_url: &TitlePath::from(title.clone()).to_string(),
        };
        let html = template.render().map_err(|_| actix_web::Error::from(()))?;
        Ok(HttpResponse::NotFound()
            .content_type("text/html")
            .body(html))
    } else {
        let page_ids = page_ids
            .iter()
            .filter(|page_id| all || !page_graph.is_obsoleted(page_id))
            .copied()
            .collect::<Vec<PageId>>();
        if page_ids.len() == 1 {
            Ok(HttpResponse::Found()
                .header(
                    actix_web::http::header::LOCATION,
                    PagePath::from(page_ids[0]).to_string(),
                )
                .finish())
        } else {
            let pages = page_ids
                .iter()
                .map(|page_id| PageItemTemplate {
                    id: page_id.to_string(),
                    obsoleted: page_graph.is_obsoleted(page_id),
                    url: PagePath::from(*page_id).to_string(),
                })
                .collect::<Vec<PageItemTemplate>>();
            let template = TitleTemplate {
                title: title.as_str(),
                title_url: &TitlePath::from(title.clone()).to_string(),
                pages: &pages,
            };
            let html = template.render().map_err(|_| actix_web::Error::from(()))?;
            Ok(HttpResponse::Ok().content_type("text/html").body(html))
        }
    }
}
