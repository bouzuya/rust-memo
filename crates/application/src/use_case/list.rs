use crate::helpers::{is_obsoleted, read_obsoleted_map};
use entity::PageId;
use use_case::{HasPageRepository, PageRepository};

#[derive(Clone)]
pub struct PageItem {
    pub id: PageId,
    pub obsoleted: bool,
}

pub fn list<App: HasPageRepository>(
    app: &App,
    all: bool,
) -> Result<Vec<PageItem>, Box<dyn std::error::Error>> {
    let obsoleted_map = read_obsoleted_map()?;
    let mut page_ids = app.page_repository().find_ids()?;
    page_ids.reverse();
    let pages = page_ids
        .into_iter()
        .map(|page_id| PageItem {
            id: page_id,
            obsoleted: is_obsoleted(&obsoleted_map, &page_id),
        })
        .filter(|template| all || !template.obsoleted)
        .collect::<Vec<PageItem>>();
    Ok(pages)
}
