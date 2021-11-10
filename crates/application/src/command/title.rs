use crate::use_case::get_title;
use entity::PageId;

pub fn title(id_like: &str) -> Result<(), Box<dyn std::error::Error>> {
    let page_id = PageId::from_like_str(id_like).expect("invalid ID format");
    let page_title = get_title(&page_id);
    println!("{}", page_title.to_string());
    Ok(())
}
