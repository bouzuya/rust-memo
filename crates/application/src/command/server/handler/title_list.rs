use std::sync::{Arc, Mutex};

use super::helpers::is_all;
use crate::template::{TitlesItemTemplate, TitlesTemplate};
use crate::url_helpers::titles_url;
use actix_web::{web, HttpResponse};
use askama::Template;
use entity::TitlePath;
use use_case::{HasListTitlesUseCase, ListTitlesUseCase};

pub async fn title_list<T: HasListTitlesUseCase>(
    req: actix_web::HttpRequest,
    data: web::Data<Arc<Mutex<T>>>,
) -> actix_web::Result<HttpResponse> {
    let app = data
        .get_ref()
        .lock()
        .map_err(|_| actix_web::Error::from(()))?;
    let all = is_all(&req);
    let titles = app
        .list_titles_use_case()
        .list_titles(all)
        .map_err(|_| actix_web::Error::from(()))?;
    let titles = titles
        .into_iter()
        .map(|title| TitlesItemTemplate {
            obsoleted: title.1,
            title: title.0.to_string(),
            url: TitlePath::from(title.0).to_string(),
        })
        .filter(|template| all || !template.obsoleted)
        .collect::<Vec<TitlesItemTemplate>>();
    let template = TitlesTemplate {
        show_all: all,
        title: &titles_url(),
        titles: &titles,
    };
    let html = template.render().map_err(|_| actix_web::Error::from(()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
