use use_case::{HasListPagesUseCase, ListPagesUseCase};

pub fn list<App: HasListPagesUseCase>(app: App, all: bool) -> anyhow::Result<()> {
    let pages = app.list_pages_use_case().list_pages(all)?;
    for page in pages {
        println!("{}.md\t{}", page.0, if page.1 { "(obsoleted)" } else { "" });
    }
    Ok(())
}
