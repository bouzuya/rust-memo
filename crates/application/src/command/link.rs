use std::str::FromStr;

use crate::url_helpers::{page_url, title_url};
use entity::{PageId, PageTitle};

pub fn link(id_like_or_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = match PageId::from_like_str(id_like_or_title) {
        Some(page_id) => page_url(&page_id),
        None => {
            let page_title = PageTitle::from_str(id_like_or_title)?;
            title_url(&page_title)
        }
    };
    println!("[{}]({})", id_like_or_title, url);
    Ok(())
}
