use entity::PageId;
use use_case::{EnsureLinksUseCase, HasEnsureLinksUseCase};

pub fn ensure_links<App: HasEnsureLinksUseCase>(app: App, id_like: &str) -> anyhow::Result<()> {
    let page_id = PageId::from_like_str(id_like)?;
    app.ensure_links_use_case().ensure_links(&page_id)
}
