use crate::{
    entity::{PageId, PageTitle},
    helpers::read_title,
};

pub fn get_title(page_id: &PageId) -> PageTitle {
    read_title(page_id)
}
