use crate::helpers::{is_obsoleted, list_ids, read_obsoleted_map};
use entity::PageId;

#[derive(Clone)]
pub struct PageItem {
    pub id: PageId,
    pub obsoleted: bool,
}

pub fn list(all: bool) -> Result<Vec<PageItem>, Box<dyn std::error::Error>> {
    let obsoleted_map = read_obsoleted_map()?;
    let mut page_ids = list_ids()?;
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
