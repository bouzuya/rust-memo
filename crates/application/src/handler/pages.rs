use crate::handler_helpers::is_all;
use crate::template::{PageItemTemplate, PagesTemplate};
use crate::url_helpers::page_url;
use crate::url_helpers::pages_url;
use actix_web::{web::Data, HttpResponse};
use askama::Template;
use use_case::HasPageRepository;

pub async fn pages<T: HasPageRepository>(
    req: actix_web::HttpRequest,
    data: Data<T>,
) -> Result<HttpResponse, actix_web::Error> {
    let app = data.get_ref();
    let all = is_all(&req);
    let pages = crate::use_case::list::list(app, all).map_err(|_| actix_web::Error::from(()))?;
    let pages = pages
        .into_iter()
        .map(|page| PageItemTemplate {
            id: page.id.to_string(),
            obsoleted: page.obsoleted,
            url: page_url(&page.id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let template = PagesTemplate {
        title: &pages_url(),
        pages: &pages,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
