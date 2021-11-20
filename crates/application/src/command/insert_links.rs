use anyhow::{anyhow, Context};
use entity::{PageContent, PageId, PageTitle, TitlePath};
use use_case::{HasPageRepository, PageRepository};

pub fn insert_links<App: HasPageRepository>(app: App, id_like: &str) -> anyhow::Result<()> {
    let page_id = PageId::from_like_str(id_like)?;
    // TODO: PageRepository::find_content(&self, page_id: &PageId) -> anyhow::Result<Option<PageContent>>
    let content = app
        .page_repository()
        .find_content(&page_id)?
        .with_context(|| anyhow!("file not found: {}", page_id))?;
    let page_content = PageContent::from(content);
    let links = page_content.broken_links();
    let mut content = page_content.to_string();
    if !links.is_empty() {
        content.push('\n');
    }
    content.push_str(
        links
            .into_iter()
            .map(|link| -> anyhow::Result<String> {
                let page_title = PageTitle::from(link.clone());
                let url = TitlePath::from(page_title).to_string();
                Ok(format!("[{}]: {}", link, url))
            })
            .collect::<anyhow::Result<Vec<String>>>()?
            .join("\n")
            .as_str(),
    );
    app.page_repository().save_content(&page_id, content)?;
    Ok(())
}
