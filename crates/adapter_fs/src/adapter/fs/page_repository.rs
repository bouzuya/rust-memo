use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use entity::{Page, PageContent, PageGraph, PageId, PageTitle};
use use_case::PageRepository;

// TODO: returns PathBuf
pub fn to_file_name(page_id: &PageId) -> String {
    format!("{}.md", page_id.to_string())
}

pub struct FsPageRepository {
    data_dir: PathBuf,
    page_graph: Arc<Mutex<PageGraph>>,
}

impl FsPageRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            page_graph: Arc::new(Mutex::new(PageGraph::default())),
        }
    }
}

impl PageRepository for FsPageRepository {
    fn destroy_cache(&self, page_id: &PageId) -> anyhow::Result<bool> {
        let mut page_graph = self.page_graph.lock().unwrap(); // TODO
                                                              // TODO: contains
        if page_graph.title(page_id).is_some() {
            page_graph.remove_page(page_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn destroy(&self, page_id: &PageId) -> anyhow::Result<bool> {
        // TODO: to_file_name should return PathBuf
        let file_name = to_file_name(page_id);
        let file_name = self.data_dir.join(file_name.as_str());
        Ok(if file_name.exists() {
            fs::remove_file(file_name)?;
            true
        } else {
            false
        })
    }

    fn find_by_id(&self, page_id: &PageId) -> anyhow::Result<Option<Page>> {
        // TODO: to_file_name should return PathBuf
        let file_name = to_file_name(page_id);
        let file_name = self.data_dir.join(file_name.as_str());
        Ok(if file_name.exists() {
            fs::read_to_string(file_name)
                .map(PageContent::from)
                .map(|page_content| Page::new(page_id.clone(), page_content))
                .map(Some)?
        } else {
            None
        })
    }

    fn find_by_obsoleted(&self, page_id: &PageId) -> BTreeSet<PageId> {
        self.page_graph.lock().unwrap().obsoleted_by(page_id)
    }

    // TODO: use iterator
    fn find_by_title(&self, page_title: &PageTitle) -> anyhow::Result<Vec<Option<Page>>> {
        self.page_graph
            .lock()
            .unwrap() // TODO
            .titled(page_title)
            .into_iter()
            .map(|page_id| self.find_by_id(&page_id))
            .collect::<anyhow::Result<Vec<Option<Page>>>>()
    }

    fn find_ids(&self) -> anyhow::Result<Vec<PageId>> {
        let mut ids = vec![];
        for res in fs::read_dir(self.data_dir.as_path())? {
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

    fn save_cache(&self, page: Page) -> anyhow::Result<()> {
        self.destroy_cache(&page.id())?;
        let mut page_graph = self.page_graph.lock().unwrap(); // TODO
        page_graph.add_page(page);
        Ok(())
    }

    fn save(&self, page: Page) -> anyhow::Result<()> {
        self.save_cache(page.clone())?;
        let file_name = to_file_name(page.id());
        let file_name = self.data_dir.join(file_name.as_str());
        Ok(fs::write(file_name, String::from(page.content().clone()))?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn destroy_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir.clone());

        let page_id = PageId::from_str("20210203T040506Z")?;
        assert_eq!(repository.destroy(&page_id)?, false);

        let file_path = data_dir.join("20210203T040506Z.md");
        fs::write(file_path.as_path(), "content")?;
        assert_eq!(repository.destroy(&page_id)?, true);

        Ok(())
    }

    #[test]
    fn find_by_id_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir.clone());

        let page_id = PageId::from_str("20210203T040506Z")?;
        assert!(repository.find_by_id(&page_id)?.is_none());

        let file_path = data_dir.join("20210203T040506Z.md");
        fs::write(file_path.as_path(), "content")?;
        assert_eq!(
            repository.find_by_id(&page_id)?,
            Some(Page::new(page_id, PageContent::from("content".to_string())))
        );

        Ok(())
    }

    #[test]
    fn find_ids_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir);

        let page_content = PageContent::from("".to_string());
        let page_id1 = PageId::from_str("20210203T040506Z")?;
        repository.save(Page::new(page_id1, page_content.clone()))?;
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        repository.save(Page::new(page_id2, page_content.clone()))?;
        let page_id3 = PageId::from_str("20210203T040508Z")?;
        repository.save(Page::new(page_id3, page_content))?;

        assert_eq!(repository.find_ids()?, vec![page_id1, page_id2, page_id3],);

        Ok(())
    }

    #[test]
    fn save_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir);

        let page_id = PageId::from_str("20210203T040506Z")?;
        assert!(repository.find_by_id(&page_id)?.is_none());
        let page_content = PageContent::from("content".to_string());
        repository.save(Page::new(page_id, page_content.clone()))?;
        assert_eq!(
            repository.find_by_id(&page_id)?,
            Some(Page::new(page_id, page_content))
        );

        Ok(())
    }
}
