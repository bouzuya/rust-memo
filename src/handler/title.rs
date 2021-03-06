use crate::entity::{PageId, PageTitle};
use crate::handler_helpers::is_all;
use crate::helpers::{read_obsoleted_map, read_title_map};
use crate::template::{PageItemTemplate, TitleNotFoundTemplate, TitleTemplate};
use crate::url_helpers::{page_url, title_url};
use actix_web::HttpResponse;
use askama::Template;

pub async fn title(req: actix_web::HttpRequest) -> std::io::Result<HttpResponse> {
    let all = is_all(&req);
    let params: (String,) = req.match_info().load().unwrap();
    let obsoleted_map = read_obsoleted_map()?;
    let title_map = read_title_map()?;
    let title = PageTitle::from_str(&params.0);
    if let Some(page_ids) = title_map.get(&title) {
        let page_ids = page_ids
            .iter()
            .filter(|page_id| all || !obsoleted_map.get(page_id).is_some())
            .map(|&x| x)
            .collect::<Vec<PageId>>();
        if page_ids.len() == 1 {
            Ok(HttpResponse::Found()
                .header(actix_web::http::header::LOCATION, page_url(&page_ids[0]))
                .finish())
        } else {
            let pages = page_ids
                .iter()
                .map(|page_id| PageItemTemplate {
                    id: page_id.to_string(),
                    obsoleted: obsoleted_map.get(page_id).is_some(),
                    url: page_url(&page_id),
                })
                .collect::<Vec<PageItemTemplate>>();
            let template = TitleTemplate {
                title: title.as_str(),
                title_url: &title_url(&title),
                pages: &pages,
            };
            let html = template.render().unwrap();
            Ok(HttpResponse::Ok().content_type("text/html").body(html))
        }
    } else {
        let template = TitleNotFoundTemplate {
            title: title.as_str(),
            title_url: &title_url(&title),
        };
        let html = template.render().unwrap();
        Ok(HttpResponse::NotFound()
            .content_type("text/html")
            .body(html))
    }
}
