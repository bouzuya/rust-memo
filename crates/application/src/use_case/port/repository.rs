use entity::PageId;

pub trait Repository {
    fn find_content(&self, page_id: &PageId) -> anyhow::Result<Option<String>>;
}

pub trait HasRepository {
    type Repository: Repository;

    fn repository(&self) -> &Self::Repository;
}
