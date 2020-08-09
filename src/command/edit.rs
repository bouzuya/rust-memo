use crate::helpers::edit_file;

pub fn edit(id_as_string: &str) -> Result<(), Box<dyn std::error::Error>> {
  let (old, new) = edit_file(id_as_string)?;
  println!("{} -> {}", old, new);
  Ok(())
}
