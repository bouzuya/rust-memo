use entity::PageId;
use use_case::{EnsureLinksUseCase, HasEnsureLinksUseCase};

pub fn ensure_links<App: HasEnsureLinksUseCase>(
    app: App,
    id_like: Option<String>,
) -> anyhow::Result<()> {
    let page_id = id_like
        .map(|id_like| PageId::from_like_str(id_like.as_str()))
        .transpose()?;
    app.ensure_links_use_case().ensure_links(page_id.as_ref())
}
