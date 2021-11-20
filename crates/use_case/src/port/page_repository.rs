use entity::{PageContent, PageId, PageTitle};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait PageRepository {
    fn find_content(&self, page_id: &PageId) -> anyhow::Result<Option<PageContent>>;

    fn find_ids(&self) -> anyhow::Result<Vec<PageId>>;

    fn find_title(&self, page_id: &PageId) -> anyhow::Result<Option<PageTitle>>;

    fn save_content(&self, page_id: &PageId, content: PageContent) -> anyhow::Result<()>;
}

pub trait HasPageRepository {
    type PageRepository: PageRepository;

    fn page_repository(&self) -> &Self::PageRepository;
}
