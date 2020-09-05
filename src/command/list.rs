use crate::helpers::{is_obsoleted, list_ids, read_obsoleted_map};

struct PageItem {
  id: String,
  obsoleted: bool,
}

pub fn list(all: bool) -> Result<(), Box<dyn std::error::Error>> {
  let obsoleted_map = read_obsoleted_map()?;
  let page_ids = list_ids()?;
  let pages = page_ids
    .iter()
    .map(|page_id| PageItem {
      id: page_id.to_string(),
      obsoleted: is_obsoleted(&obsoleted_map, &page_id),
    })
    .filter(|template| all || !template.obsoleted)
    .collect::<Vec<PageItem>>();
  for page in pages {
    println!(
      "{}.md\t{}",
      page.id,
      if page.obsoleted { "(obsoleted)" } else { "" }
    );
  }
  Ok(())
}
