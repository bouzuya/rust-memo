use entity::{PageId, PagePath, PageTitle, TitlePath};

pub fn link(id_like_or_title: &str) -> anyhow::Result<()> {
    let url = match PageId::from_like_str(id_like_or_title) {
        Ok(page_id) => PagePath::from(page_id).to_string(),
        Err(_) => {
            let page_title = PageTitle::from(id_like_or_title.to_string());
            TitlePath::from(page_title).to_string()
        }
    };
    println!("[{}]({})", id_like_or_title, url);
    Ok(())
}
