use entity::Query;
use use_case::{HasPageRepository, PageRepository};

use crate::helpers::to_file_name;

pub fn search<App: HasPageRepository>(app: App, query: Query) -> anyhow::Result<()> {
    let matches = app.page_repository().find_by_query(&query)?;
    for (page_id, line, col) in matches {
        println!("{}:{}:{}", to_file_name(&page_id), line, col);
    }
    Ok(())
}
