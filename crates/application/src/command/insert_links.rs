use anyhow::{anyhow, Context};
use entity::PageId;
use use_case::{HasPageRepository, PageRepository};

pub fn insert_links<App: HasPageRepository>(app: App, id_like: &str) -> anyhow::Result<()> {
    let page_id = PageId::from_like_str(id_like)?;
    let mut page_content = app
        .page_repository()
        .find_content(&page_id)?
        .with_context(|| anyhow!("file not found: {}", page_id))?;
    page_content.ensure_links();
    app.page_repository().save_content(&page_id, page_content)?;
    Ok(())
}
