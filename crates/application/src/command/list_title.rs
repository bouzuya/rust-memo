use use_case::{HasListTitlesUseCase, ListTitlesUseCase};

pub fn list_title<App: HasListTitlesUseCase>(app: App, all: bool) -> anyhow::Result<()> {
    let titles = app.list_titles_use_case().list_titles(all)?;
    for title in titles {
        println!("{}\t{}", title.0, if title.1 { "(obsoleted)" } else { "" });
    }
    Ok(())
}
