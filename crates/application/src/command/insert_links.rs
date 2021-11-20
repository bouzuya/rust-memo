use anyhow::{anyhow, Context};
use entity::{PageContent, PageId};
use use_case::{HasPageRepository, PageRepository};

pub fn insert_links<App: HasPageRepository>(app: App, id_like: &str) -> anyhow::Result<()> {
    let page_id = PageId::from_like_str(id_like)?;
    // TODO: PageRepository::find_content(&self, page_id: &PageId) -> anyhow::Result<Option<PageContent>>
    let content = app
        .page_repository()
        .find_content(&page_id)?
        .with_context(|| anyhow!("file not found: {}", page_id))?;
    let mut page_content = PageContent::from(content);
    page_content.ensure_links();
    let content = String::from(page_content);
    app.page_repository().save_content(&page_id, content)?;
    Ok(())
}
