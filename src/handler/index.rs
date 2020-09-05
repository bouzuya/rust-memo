use crate::url_helpers::pages_url;
use actix_web::HttpResponse;

pub async fn index() -> impl actix_web::Responder {
  HttpResponse::Found()
    .header(actix_web::http::header::LOCATION, pages_url())
    .finish()
}
