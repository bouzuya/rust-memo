use crate::template::IndexTemplate;
use actix_web::HttpResponse;
use askama::Template;

pub async fn index() -> impl actix_web::Responder {
  let template = IndexTemplate {};
  let html = template.render().unwrap();
  HttpResponse::Ok().content_type("text/html").body(html)
}
