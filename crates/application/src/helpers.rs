use entity::PageId;

// TODO: returns PathBuf
pub fn to_file_name(page_id: &PageId) -> String {
    format!("{}.md", page_id.to_string())
}
