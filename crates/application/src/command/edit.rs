use entity::PageIdOrPageTitle;
use use_case::{EditPageUseCase, HasEditPageUseCase};

use crate::helpers::to_file_name;

pub fn edit<T: HasEditPageUseCase>(
    app: T,
    id_like_or_title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let page_id_or_page_title = PageIdOrPageTitle::from(id_like_or_title);
    let (old_page_id, new_page_id) = app.edit_page_use_case().edit_page(&page_id_or_page_title)?;
    let old_file_name = to_file_name(&old_page_id);
    let new_file_name = to_file_name(&new_page_id);
    println!("{} -> {}", old_file_name, new_file_name);
    Ok(())
}
