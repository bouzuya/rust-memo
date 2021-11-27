use use_case::HasPageRepository;

pub fn list<App: HasPageRepository>(app: App, all: bool) -> anyhow::Result<()> {
    let pages = crate::use_case::list::list(&app, all)?;
    for page in pages {
        println!(
            "{}.md\t{}",
            page.id.to_string(),
            if page.obsoleted { "(obsoleted)" } else { "" }
        );
    }
    Ok(())
}
