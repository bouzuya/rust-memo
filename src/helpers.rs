use crate::entity::{PageId, PageTitle};
use crate::url_helpers::page_url;

pub fn to_file_name(page_id: &PageId) -> String {
    format!("{}.md", page_id.to_string())
}

pub fn read_linked_map(
) -> std::io::Result<std::collections::BTreeMap<PageTitle, std::collections::BTreeSet<PageId>>> {
    let mut map = std::collections::BTreeMap::new();
    let page_ids = list_ids()?;
    for &from_page_id in page_ids.iter() {
        let tos = read_links(&from_page_id)?;
        for to_page_title in tos.into_iter() {
            map.entry(to_page_title)
                .or_insert(std::collections::BTreeSet::new())
                .insert(from_page_id);
        }
    }
    Ok(map)
}

pub fn read_links(page_id: &PageId) -> std::io::Result<Vec<PageTitle>> {
    let file_name = to_file_name(page_id);
    let content = std::fs::read_to_string(&file_name)?;
    let links = read_links_impl(&content);
    Ok(links
        .iter()
        .map(|s| PageTitle::from_str(s))
        .collect::<Vec<PageTitle>>())
}

fn read_links_impl(md: &str) -> Vec<String> {
    pulldown_cmark::Parser::new(md)
        .filter_map(|event| match event {
            pulldown_cmark::Event::End(tag) => Some(tag),
            _ => None,
        })
        .filter_map(|tag| match tag {
            pulldown_cmark::Tag::Link(_, to, _) => Some(to),
            _ => None,
        })
        .filter_map(|to| {
            to.strip_prefix("/titles/")
                .and_then(|s| percent_encoding::percent_decode_str(s).decode_utf8().ok())
                .map(|s| s.to_string())
        })
        .collect::<Vec<String>>()
}

fn read_obsoletes(page_id: &PageId) -> Vec<PageId> {
    use regex::Regex;
    let re = Regex::new(r"^- \[(\d{4}\d{2}\d{2}T\d{2}\d{2}\d{2}Z)\]\(.*\)$").unwrap();
    let file_name = to_file_name(&page_id);
    let content = match std::fs::read_to_string(&file_name) {
        Ok(x) => x,
        Err(_) => return Vec::new(),
    };
    if let Some(index) = content.find("\n## Obsoletes") {
        let mut obsoletes = Vec::new();
        for line in content[index..].lines() {
            if let Some(caps) = re.captures(line) {
                let s = caps.get(1).unwrap().as_str();
                if let Some(page_id) = PageId::from_str(s) {
                    obsoletes.push(page_id);
                }
            }
        }
        obsoletes
    } else {
        return Vec::new();
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
                .or_insert(std::collections::BTreeSet::new())
                .insert(new_page_id);
        }
    }
    Ok(map)
}

pub fn read_title(page_id: &PageId) -> PageTitle {
    use std::io::prelude::*;
    let file = match std::fs::File::open(&to_file_name(page_id)) {
        Ok(file) => file,
        Err(_) => return PageTitle::empty(),
    };
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => {}
        Err(_) => return PageTitle::empty(),
    };
    if buffer.starts_with("# ") {
        return PageTitle::from_str(buffer[2..].trim());
    } else {
        return PageTitle::empty();
    }
}

pub fn read_title_map() -> std::io::Result<std::collections::BTreeMap<PageTitle, Vec<PageId>>> {
    let mut title_map = std::collections::BTreeMap::new();
    let page_ids = list_ids()?;
    for &page_id in page_ids.iter() {
        let title = read_title(&page_id);
        title_map.entry(title).or_insert(vec![]).push(page_id);
    }
    Ok(title_map)
}

pub fn list_ids() -> std::io::Result<Vec<PageId>> {
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
        if let Some(page_id) = PageId::from_str(id_as_string) {
            ids.push(page_id);
        }
    }
    ids.sort();
    Ok(ids)
}

pub fn create_new_file(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::prelude::*;
    let page_id = PageId::new().expect("This application is out of date.");
    let file_name = to_file_name(&page_id);
    let mut file = std::fs::File::create(&file_name)?;
    writeln!(file, "{}", content)?;
    file.flush()?;
    Ok(file_name)
}

pub fn edit_file(id_like_string: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let page_id = PageId::from_like_str(id_like_string).expect("invalid ID format");
    let old_file_name = to_file_name(&page_id);
    let mut content = std::fs::read_to_string(&old_file_name)?;
    if let Some(index) = content.find("\n## Obsoletes") {
        content.truncate(index);
    }
    content.push_str(&format!(
        "\n## Obsoletes\n\n- [{}]({})",
        page_id.to_string(),
        page_url(&page_id)
    ));
    let new_file_name = create_new_file(&content)?;
    Ok((old_file_name, new_file_name))
}

pub fn is_obsoleted(
    obsoleted_map: &std::collections::BTreeMap<PageId, std::collections::BTreeSet<PageId>>,
    page_id: &PageId,
) -> bool {
    obsoleted_map.get(&page_id).is_some()
}

#[test]
fn cmark_test() {
    let md = "
[inline](/titles/bouzuya)
[ref][ref]
[ref_unknown][ref_unknown]
[collapsed][]
[collapsed_unknown][]
[shortcut]
[shortcut_unknown]
</titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%844>

[ref]: /titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%841
[collapsed]: /titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%842
[shortcut]: /titles/%E3%81%BC%E3%81%86%E3%81%9A%E3%82%843
";
    let links = read_links_impl(&md);
    assert_eq!(
        links,
        vec!["bouzuya", "ぼうずや1", "ぼうずや2", "ぼうずや3"]
    );
}
