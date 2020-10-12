use serde::Serialize;

#[derive(Default, Clone, Serialize, dynomite::Item)]
pub struct DownloadCategory {
    #[serde(skip)]
    #[dynomite(partition_key)]
    #[dynomite(rename = "type")]
    r#type: String,

    #[dynomite(sort_key)]
    id: String,

    title: String,
    description: String,
}

#[derive(Default, Clone, Serialize, dynomite::Item)]
pub struct Download {
    #[serde(skip)]
    #[dynomite(partition_key)]
    #[dynomite(rename = "type")]
    r#type: String,

    #[dynomite(sort_key)]
    id: String,

    name: String,
    category: String,
    url: String,
    description: String,

    #[dynomite(default)]
    version: Option<String>,
}
