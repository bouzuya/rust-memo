use crate::url_helpers::title_url;
use entity::{PageId, PagePath, PageTitle};

pub fn link(id_like_or_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = match PageId::from_like_str(id_like_or_title) {
        Ok(page_id) => PagePath::from(page_id).to_string(),
        Err(_) => {
            let page_title = PageTitle::from(id_like_or_title.to_string());
            title_url(&page_title)
        }
    };
    println!("[{}]({})", id_like_or_title, url);
    Ok(())
}
