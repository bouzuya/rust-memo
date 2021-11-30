use std::str::FromStr;

use entity::{PageContent, PageId};

// TODO: returns PathBuf
pub fn to_file_name(page_id: &PageId) -> String {
    format!("{}.md", page_id.to_string())
}

fn read_obsoletes(page_id: &PageId) -> Vec<PageId> {
    let file_name = to_file_name(page_id);
    let content = match std::fs::read_to_string(&file_name) {
        Ok(x) => x,
        Err(_) => return Vec::new(),
    };
    let page_content = PageContent::from(content);
    page_content.obsoletes()
}

pub fn read_obsoleted_map(
) -> std::io::Result<std::collections::BTreeMap<PageId, std::collections::BTreeSet<PageId>>> {
    let mut map = std::collections::BTreeMap::new();
    let page_ids = list_ids()?;
    for &new_page_id in page_ids.iter() {
        let obsoletes = read_obsoletes(&new_page_id);
        for &old_page_id in obsoletes.iter() {
            map.entry(old_page_id)
                .or_insert_with(std::collections::BTreeSet::new)
                .insert(new_page_id);
        }
    }
    Ok(map)
}

fn list_ids() -> std::io::Result<Vec<PageId>> {
    let mut ids = vec![];
    for res in std::fs::read_dir(".")? {
        let dir_entry = res?;
        let file_type = dir_entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }
        let path = dir_entry.path();
        let id_as_string = match path.file_stem().and_then(|os_str| os_str.to_str()) {
            Some(x) => x,
            None => continue,
        };
        if let Ok(page_id) = PageId::from_str(id_as_string) {
            ids.push(page_id);
        }
    }
    ids.sort();
    Ok(ids)
}

pub fn is_obsoleted(
    obsoleted_map: &std::collections::BTreeMap<PageId, std::collections::BTreeSet<PageId>>,
    page_id: &PageId,
) -> bool {
    obsoleted_map.get(page_id).is_some()
}
