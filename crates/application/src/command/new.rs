use std::str::FromStr;

use entity::PageTitle;
use use_case::{HasNewPageUseCase, NewPageUseCase};

use crate::helpers::to_file_name;

pub fn new<App: HasNewPageUseCase>(
    app: App,
    title: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let title = PageTitle::from_str(title.unwrap_or_default())?;
    let new_page_id = app.new_page_use_case().new_page(title)?;
    let new_file_name = to_file_name(&new_page_id);
    println!("{}", new_file_name);
    Ok(())
}
