use crate::handler_helpers::is_all;
use crate::template::{TitlesItemTemplate, TitlesTemplate};
use crate::url_helpers::titles_url;
use actix_web::HttpResponse;
use askama::Template;
use entity::TitlePath;

pub async fn titles(req: actix_web::HttpRequest) -> actix_web::Result<HttpResponse> {
    let all = is_all(&req);
    let titles =
        crate::use_case::list_title::list_title(all).map_err(|_| actix_web::Error::from(()))?;
    let titles = titles
        .into_iter()
        .map(|title| TitlesItemTemplate {
            obsoleted: title.obsoleted,
            title: title.title.to_string(),
            url: TitlePath::from(title.title).to_string(),
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
