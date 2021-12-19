use std::sync::{Arc, Mutex};

use actix_web::{http, web, HttpResponse};
use anyhow::Context;
use entity::{Page, PageContent, PageId};
use use_case::{HasPageRepository, PageRepository};

#[derive(serde::Deserialize)]
pub struct FormData {
    content: String,
}

pub async fn page_create<T: HasPageRepository>(
    data: web::Data<Arc<Mutex<T>>>,
    form: web::Form<FormData>,
) -> actix_web::Result<HttpResponse> {
    let app = data
        .get_ref()
        .lock()
        .map_err(|_| actix_web::Error::from(()))?;
    let page_id = PageId::new()
        .context("This application is out of date")
        .map_err(|_| actix_web::Error::from(()))?;
    let page_content = PageContent::from(form.0.content);
    // TODO: UseCase
    let page = Page::new(page_id, page_content);
    app.page_repository()
        .save(page)
        .map_err(|_| actix_web::Error::from(()))?;
    Ok(HttpResponse::SeeOther()
        .set_header(http::header::LOCATION, format!("/pages/{}", page_id))
        .body(""))
}
