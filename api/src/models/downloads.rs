use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DownloadCategory {
    id: String,

    title: String,
    description: String,
}

impl From<DbDownloadCategory> for DownloadCategory {
    fn from(db: DbDownloadCategory) -> Self {
        db.download_category
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DbDownloadCategory {
    #[serde(rename = "type")]
    r#type: String,

    #[serde(flatten)]
    download_category: DownloadCategory,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Download {
    id: String,

    name: String,
    category: String,
    url: String,
    description: String,

    #[serde(default)]
    version: Option<String>,
}

impl From<DbDownload> for Download {
    fn from(db: DbDownload) -> Self {
        db.download
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DbDownload {
    #[serde(rename = "type")]
    r#type: String,

    #[serde(flatten)]
    download: Download,
}
