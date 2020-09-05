use crate::handler_helpers::is_all;
use crate::helpers::{is_obsoleted, read_obsoleted_map, read_title_map};
use crate::template::{TitlesItemTemplate, TitlesTemplate};
use crate::url_helpers::{title_url, titles_url};
use actix_web::HttpResponse;
use askama::Template;

pub async fn titles(req: actix_web::HttpRequest) -> std::io::Result<HttpResponse> {
    let all = is_all(&req);
    let obsoleted_map = read_obsoleted_map()?;
    let title_map = read_title_map()?;
    let titles = title_map
        .iter()
        .map(|(title, page_ids)| TitlesItemTemplate {
            obsoleted: !page_ids
                .iter()
                .any(|page_id| !is_obsoleted(&obsoleted_map, page_id)),
            title: title.to_string(),
            url: title_url(&title),
        })
        .filter(|template| all || !template.obsoleted)
        .collect::<Vec<TitlesItemTemplate>>();
    let template = TitlesTemplate {
        show_all: all,
        title: &titles_url(),
        titles: &titles,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
