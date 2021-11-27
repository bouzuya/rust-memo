use crate::template::IndexTemplate;
use actix_web::HttpResponse;
use askama::Template;

pub async fn index() -> actix_web::Result<HttpResponse> {
    let template = IndexTemplate {};
    let html = template.render().map_err(|_| actix_web::Error::from(()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
