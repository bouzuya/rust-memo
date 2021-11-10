pub fn list(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let pages = crate::use_case::list::list(all)?;
    for page in pages {
        println!(
            "{}.md\t{}",
            page.id.to_string(),
            if page.obsoleted { "(obsoleted)" } else { "" }
        );
    }
    Ok(())
}
