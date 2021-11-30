use entity::PageId;
use use_case::{HasPageRepository, PageRepository};

#[derive(Clone)]
pub struct PageItem {
    pub id: PageId,
    pub obsoleted: bool,
}

pub fn list<App: HasPageRepository>(app: &App, all: bool) -> anyhow::Result<Vec<PageItem>> {
    let page_graph = app.page_repository().load_page_graph()?;
    let mut page_ids = app.page_repository().find_ids()?;
    page_ids.reverse();
    let pages = page_ids
        .into_iter()
        .map(|page_id| PageItem {
            id: page_id,
            obsoleted: page_graph.is_obsoleted(&page_id),
        })
        .filter(|template| all || !template.obsoleted)
        .collect::<Vec<PageItem>>();
    Ok(pages)
}
