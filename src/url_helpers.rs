use crate::page_id::PageId;
use crate::page_title::PageTitle;

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

pub fn title_url(title: &PageTitle) -> String {
    format!(
        "{}/{}",
        titles_url(),
        percent_encoding::utf8_percent_encode(title.as_str(), percent_encoding::NON_ALPHANUMERIC)
    )
}

pub fn titles_url() -> String {
    format!("/titles")
}
