use async_std::io::WriteExt;
use async_std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Db {
    #[serde(skip)]
    path: async_std::path::PathBuf,
    pub content: String,
}

impl Db {
    pub fn new(path: impl AsRef<async_std::path::Path>) -> Self {
        Self {
            path: path.as_ref().into(),
            content: "".to_owned(),
        }
    }

    pub async fn read(&mut self) -> crate::Res<()> {
        let bytes = async_std::fs::read(&self.path).await?;
        let slice = bytes.as_slice();
        let str = std::str::from_utf8(slice).unwrap().to_owned();

        self.content = str;

        Ok(())
    }

    pub fn serialize(&mut self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.content).into()
    }

    pub fn deserialize(&mut self) -> serde_json::Result<Vec<crate::Post>> {
        serde_json::from_str(&self.content).into()
    }

    pub async fn write_back(&self, posts: &Vec<crate::Post>) -> crate::Res<()> {
        let serialized = serde_json::to_string_pretty(&posts)?;
        let mut file = async_std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .await?;

        writeln!(file, "{serialized}").await?;

        Ok(())
    }

    ///  Synchronised means that the content field will also be updated, and will be same to the file.
    pub async fn write_back_sync(&mut self, posts: &Vec<crate::Post>) -> crate::Res<()> {
        self.write_back(posts).await?;
        self.read().await?;

        Ok(())
    }
}
