pub fn list_title(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let titles = crate::use_case::list_title::list_title(all)?;
    for title in titles {
        println!(
            "{}\t{}",
            title.title.to_string(),
            if title.obsoleted { "(obsoleted)" } else { "" }
        );
    }
    Ok(())
}
