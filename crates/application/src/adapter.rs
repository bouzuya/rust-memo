use std::{fs, path::PathBuf};

use entity::PageId;

use crate::{helpers::to_file_name, use_case::Repository};

pub struct FsRepository {
    data_dir: PathBuf,
}

impl FsRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }
}

impl Repository for FsRepository {
    fn find_content(&self, page_id: &PageId) -> anyhow::Result<Option<String>> {
        // TODO: to_file_name should return PathBuf
        let file_name = to_file_name(page_id);
        let file_name = self.data_dir.join(file_name.as_str());
        Ok(if file_name.exists() {
            fs::read_to_string(file_name).map(Some)?
        } else {
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().to_path_buf();
        let repository = FsRepository::new(data_dir.clone());

        let page_id = PageId::from_str("20210203T040506Z")?;
        assert!(repository.find_content(&page_id)?.is_none());

        let file_path = data_dir.join("20210203T040506Z.md");
        fs::write(file_path.as_path(), "content")?;
        assert_eq!(
            repository.find_content(&page_id)?,
            Some("content".to_string())
        );

        Ok(())
    }
}
