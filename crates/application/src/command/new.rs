use crate::helpers::create_new_file;

pub fn new(title: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let new = create_new_file(&format!("# {}", title.unwrap_or("")))?;
    println!("{}", new);
    Ok(())
}
