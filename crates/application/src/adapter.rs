use std::{fs, path::PathBuf, str::FromStr};

use entity::{PageContent, PageId, PageTitle};
use use_case::PageRepository;

use crate::helpers::to_file_name;

pub struct FsPageRepository {
    data_dir: PathBuf,
}

impl FsPageRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }
}

impl PageRepository for FsPageRepository {
    fn find_content(&self, page_id: &PageId) -> anyhow::Result<Option<PageContent>> {
        // TODO: to_file_name should return PathBuf
        let file_name = to_file_name(page_id);
        let file_name = self.data_dir.join(file_name.as_str());
        Ok(if file_name.exists() {
            fs::read_to_string(file_name)
                .map(PageContent::from)
                .map(Some)?
        } else {
            None
        })
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

    fn find_title(&self, page_id: &PageId) -> anyhow::Result<Option<PageTitle>> {
        // TODO: PageContent::title
        let content = match self.find_content(page_id)? {
            Some(x) => String::from(x),
            None => return Ok(None),
        };
        let first_line = match content.lines().next() {
            Some(x) => x,
            None => return Ok(None),
        };
        let title = match first_line.strip_prefix("# ") {
            Some(x) => x,
            None => return Ok(None),
        };
        Ok(PageTitle::from_str(title).map(Some)?)
    }

    fn save_content(&self, page_id: &PageId, content: String) -> anyhow::Result<()> {
        let file_name = to_file_name(page_id);
        let file_name = self.data_dir.join(file_name.as_str());
        Ok(fs::write(file_name, content)?)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn find_content_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir.clone());

        let page_id = PageId::from_str("20210203T040506Z")?;
        assert!(repository.find_content(&page_id)?.is_none());

        let file_path = data_dir.join("20210203T040506Z.md");
        fs::write(file_path.as_path(), "content")?;
        assert_eq!(
            repository.find_content(&page_id)?,
            Some(PageContent::from("content".to_string()))
        );

        Ok(())
    }

    #[test]
    fn find_ids_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir);

        let page_id1 = PageId::from_str("20210203T040506Z")?;
        repository.save_content(&page_id1, "".to_string())?;
        let page_id2 = PageId::from_str("20210203T040507Z")?;
        repository.save_content(&page_id2, "".to_string())?;
        let page_id3 = PageId::from_str("20210203T040508Z")?;
        repository.save_content(&page_id3, "".to_string())?;

        assert_eq!(repository.find_ids()?, vec![page_id1, page_id2, page_id3],);

        Ok(())
    }

    #[test]
    fn find_title_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir.clone());

        let page_id = PageId::from_str("20210203T040506Z")?;
        assert!(repository.find_content(&page_id)?.is_none());

        let file_path = data_dir.join("20210203T040506Z.md");
        fs::write(file_path.as_path(), "# title1\n\ncontent")?;
        assert_eq!(
            repository.find_title(&page_id)?,
            Some(PageTitle::from("title1".to_string()))
        );

        Ok(())
    }

    #[test]
    fn save_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsPageRepository::new(data_dir);

        let page_id = PageId::from_str("20210203T040506Z")?;
        assert!(repository.find_content(&page_id)?.is_none());

        repository.save_content(&page_id, "content".to_string())?;
        assert_eq!(
            repository.find_content(&page_id)?,
            Some(PageContent::from("content".to_string()))
        );

        Ok(())
    }
}
