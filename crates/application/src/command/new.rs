use use_case::HasRepository;

use crate::helpers::{create_new_file, to_file_name};

pub fn new<App: HasRepository>(
    app: App,
    title: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_page_id = create_new_file(app, format!("# {}", title.unwrap_or("")))?;
    let new_file_name = to_file_name(&new_page_id);
    println!("{}", new_file_name);
    Ok(())
}
