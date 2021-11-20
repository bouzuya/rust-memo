use std::str::FromStr;

use entity::{PageContent, PageId, PageTitle};

// TODO: returns PathBuf
pub fn to_file_name(page_id: &PageId) -> String {
    format!("{}.md", page_id.to_string())
}

pub fn read_linked_map(
) -> std::io::Result<std::collections::BTreeMap<PageTitle, std::collections::BTreeSet<PageId>>> {
    let mut map = std::collections::BTreeMap::new();
    let page_ids = list_ids()?;
    for &from_page_id in page_ids.iter() {
        // TODO: use PageRepository::find_content
        let file_name = to_file_name(&from_page_id);
        let content = std::fs::read_to_string(&file_name)?;
        let tos = PageContent::from(content).title_links();
        for to_page_title in tos.into_iter() {
            map.entry(to_page_title)
                .or_insert_with(std::collections::BTreeSet::new)
                .insert(from_page_id);
        }
    }
    Ok(map)
}

fn read_obsoletes(page_id: &PageId) -> Vec<PageId> {
    use regex::Regex;
    let re = Regex::new(r"^- \[(\d{4}\d{2}\d{2}T\d{2}\d{2}\d{2}Z)\]\(.*\)$").unwrap();
    let file_name = to_file_name(page_id);
    let content = match std::fs::read_to_string(&file_name) {
        Ok(x) => x,
        Err(_) => return Vec::new(),
    };
    if let Some(index) = content.find("\n## Obsoletes") {
        let mut obsoletes = Vec::new();
        for line in content[index..].lines() {
            if let Some(caps) = re.captures(line) {
                let s = caps.get(1).unwrap().as_str();
                if let Ok(page_id) = PageId::from_str(s) {
                    obsoletes.push(page_id);
                }
            }
        }
        obsoletes
    } else {
        Vec::new()
    }
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

fn read_title(page_id: &PageId) -> PageTitle {
    use std::io::prelude::*;
    let file = match std::fs::File::open(&to_file_name(page_id)) {
        Ok(file) => file,
        Err(_) => return PageTitle::default(),
    };
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => {}
        Err(_) => return PageTitle::default(),
    };
    if let Some(stripped) = buffer.strip_prefix("# ") {
        PageTitle::from(stripped.trim().to_string())
    } else {
        PageTitle::default()
    }
}

pub fn read_title_map() -> std::io::Result<std::collections::BTreeMap<PageTitle, Vec<PageId>>> {
    let mut title_map = std::collections::BTreeMap::new();
    let page_ids = list_ids()?;
    for &page_id in page_ids.iter() {
        let title = read_title(&page_id);
        title_map
            .entry(title)
            .or_insert_with(Vec::new)
            .push(page_id);
    }
    Ok(title_map)
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
