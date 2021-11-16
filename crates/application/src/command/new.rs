use std::str::FromStr;

use anyhow::Context;
use entity::{PageId, PageTitle};
use use_case::{HasPageRepository, PageRepository};

use crate::helpers::to_file_name;

fn create_page_use_case<App: HasPageRepository>(
    app: App,
    title: PageTitle,
) -> anyhow::Result<PageId> {
    let page_id = PageId::new().context("This application is out of date.")?;
    let content = format!("# {}", title);
    app.page_repository()
        .save_content(&page_id, content)
        .map(|_| page_id)
}

pub fn new<App: HasPageRepository>(
    app: App,
    title: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let title = PageTitle::from_str(title.unwrap_or_default())?;
    let new_page_id = create_page_use_case(app, title)?;
    let new_file_name = to_file_name(&new_page_id);
    println!("{}", new_file_name);
    Ok(())
}
