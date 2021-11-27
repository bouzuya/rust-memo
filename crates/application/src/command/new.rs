use entity::PageTitle;
use use_case::{HasNewPageUseCase, NewPageUseCase};

use crate::helpers::to_file_name;

pub fn new<App: HasNewPageUseCase>(app: App, title: Option<&str>) -> anyhow::Result<()> {
    let title = PageTitle::from(title.unwrap_or_default().to_string());
    let (new_page_id, already_exists) = app.new_page_use_case().new_page(title.clone())?;
    // TODO: use presenter
    let new_file_name = to_file_name(&new_page_id);
    println!("{}", new_file_name);
    if already_exists {
        println!("{} already_exists", title)
    }
    Ok(())
}
