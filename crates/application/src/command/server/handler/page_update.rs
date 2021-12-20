use std::sync::{Arc, Mutex};

use actix_web::{http, web, HttpResponse};
use entity::{Page, PageContent, PageId};
use use_case::{HasPageRepository, PageRepository};

#[derive(serde::Deserialize)]
pub struct PageUpdatePath {
    page_id: String,
}

#[derive(serde::Deserialize)]
pub struct PageUpdateForm {
    content: String,
}

pub async fn page_update<T: HasPageRepository>(
    data: web::Data<Arc<Mutex<T>>>,
    path: web::Path<PageUpdatePath>,
    form: web::Form<PageUpdateForm>,
) -> actix_web::Result<HttpResponse> {
    let app = data
        .get_ref()
        .lock()
        .map_err(|_| actix_web::Error::from(()))?;
    let page_id =
        PageId::from_like_str(path.page_id.as_str()).map_err(|_| actix_web::Error::from(()))?;
    let page_content = PageContent::from(form.into_inner().content);
    // TODO: UseCase
    let page = Page::new(page_id, page_content);
    app.page_repository()
        .save(page)
        .map_err(|_| actix_web::Error::from(()))?;
    Ok(HttpResponse::SeeOther()
        .set_header(http::header::LOCATION, format!("/pages/{}", page_id))
        .body(""))
}
