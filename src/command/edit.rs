use crate::helpers::edit_file;

pub fn edit(id_like_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (old, new) = edit_file(id_like_string)?;
    println!("{} -> {}", old, new);
    Ok(())
}
