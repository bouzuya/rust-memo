use entity::Query;
use use_case::{HasPageRepository, PageRepository};

use crate::helpers::to_file_name;

pub fn search<App: HasPageRepository>(app: App, query: Query, all: bool) -> anyhow::Result<()> {
    let page_graph = app.page_repository().load_page_graph()?;
    let matches = app.page_repository().find_by_query(&query)?;
    for (page_id, line, col) in matches {
        if !all && page_graph.is_obsoleted(&page_id) {
            continue;
        }
        println!("{}:{}:{}", to_file_name(&page_id), line, col);
    }
    Ok(())
}
