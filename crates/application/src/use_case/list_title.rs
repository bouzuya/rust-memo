use crate::helpers::{is_obsoleted, read_obsoleted_map, read_title_map};
use entity::PageTitle;

#[derive(Clone)]
pub struct TitleItem {
    pub title: PageTitle,
    pub obsoleted: bool,
}

pub fn list_title(all: bool) -> Result<Vec<TitleItem>, Box<dyn std::error::Error>> {
    let obsoleted_map = read_obsoleted_map()?;
    let title_map = read_title_map()?;
    let titles = title_map
        .into_iter()
        .map(|(title, page_ids)| TitleItem {
            obsoleted: !page_ids
                .iter()
                .any(|page_id| !is_obsoleted(&obsoleted_map, page_id)),
            title,
        })
        .filter(|template| all || !template.obsoleted)
        .collect::<Vec<TitleItem>>();
    Ok(titles)
}
