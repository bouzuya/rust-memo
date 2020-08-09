use crate::page_id::PageId;

pub fn page_url(page_id: &PageId) -> String {
    format!(
        "{}/{}",
        pages_url(),
        percent_encoding::utf8_percent_encode(
            &page_id.to_string(),
            percent_encoding::NON_ALPHANUMERIC,
        )
    )
}

pub fn pages_url() -> String {
    format!("/pages")
}

pub fn title_url(title: &str) -> String {
    format!(
        "{}/{}",
        titles_url(),
        percent_encoding::utf8_percent_encode(title, percent_encoding::NON_ALPHANUMERIC)
    )
}

pub fn titles_url() -> String {
    format!("/titles")
}
