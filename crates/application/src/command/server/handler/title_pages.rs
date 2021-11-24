use crate::template::{PageItemTemplate, TitleNotFoundTemplate, TitlePagesTemplate};
use actix_web::{web, HttpResponse};
use askama::Template;
use entity::{PagePath, PageTitle, TitlePath};
use use_case::{HasPageRepository, PageRepository};

pub async fn title_pages<T: HasPageRepository>(
    req: actix_web::HttpRequest,
    data: web::Data<T>,
) -> std::io::Result<HttpResponse> {
    let app = data.get_ref();
    let params: (String,) = req.match_info().load().unwrap();
    let page_graph = app.page_repository().load_page_graph().unwrap(); // TODO: unwrap
    let title = PageTitle::from(params.0);
    let page_ids = page_graph.titled(&title);
    if page_ids.is_empty() {
        let template = TitleNotFoundTemplate {
            title: title.as_str(),
            title_url: &TitlePath::from(title.clone()).to_string(),
        };
        let html = template.render().unwrap();
        Ok(HttpResponse::NotFound()
            .content_type("text/html")
            .body(html))
    } else {
        let pages = page_ids
            .iter()
            .map(|page_id| PageItemTemplate {
                id: page_id.to_string(),
                obsoleted: page_graph.is_obsoleted(page_id),
                url: PagePath::from(*page_id).to_string(),
            })
            .collect::<Vec<PageItemTemplate>>();
        let template = TitlePagesTemplate {
            title: title.as_str(),
            title_url: &TitlePath::from(title.clone()).to_string(),
            pages: &pages,
        };
        let html = template.render().unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    }
}
