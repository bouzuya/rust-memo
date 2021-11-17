use entity::PageId;
use use_case::{EditUseCase, HasEditUseCase};

use crate::helpers::to_file_name;

pub fn edit<T: HasEditUseCase>(
    app: T,
    id_like_string: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let old_page_id = PageId::from_like_str(id_like_string)?;
    let new_page_id = app.edit_use_case().edit(&old_page_id)?;
    let old_file_name = to_file_name(&old_page_id);
    let new_file_name = to_file_name(&new_page_id);
    println!("{} -> {}", old_file_name, new_file_name);
    Ok(())
}
