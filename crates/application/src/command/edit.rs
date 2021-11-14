use entity::PageId;

use crate::{
    helpers::{edit_file, to_file_name},
    use_case::HasRepository,
};

pub fn edit<T: HasRepository>(
    app: T,
    id_like_string: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let page_id = PageId::from_like_str(id_like_string)?;
    let (old_page_id, new_page_id) = edit_file(app, page_id)?;
    let old_file_name = to_file_name(&old_page_id);
    let new_file_name = to_file_name(&new_page_id);
    println!("{} -> {}", old_file_name, new_file_name);
    Ok(())
}
