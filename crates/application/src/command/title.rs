use entity::PageId;
use use_case::{HasPageRepository, PageRepository};

pub fn title<App: HasPageRepository>(
    app: App,
    id_like: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let page_id = PageId::from_like_str(id_like).expect("invalid ID format");
    let page_title = app
        .page_repository()
        .find_title(&page_id)
        .unwrap_or(None)
        .unwrap_or_default();
    println!("{}", page_title.to_string());
    Ok(())
}
