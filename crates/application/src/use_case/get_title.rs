use crate::helpers::read_title;
use entity::{PageId, PageTitle};

pub fn get_title(page_id: &PageId) -> PageTitle {
    read_title(page_id)
}
