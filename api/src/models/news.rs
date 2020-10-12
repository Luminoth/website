use serde::Serialize;

#[derive(Default, Clone, Serialize, dynomite::Item)]
pub struct NewsAuthor {
    #[serde(skip)]
    #[dynomite(partition_key)]
    #[dynomite(rename = "type")]
    r#type: String,

    #[dynomite(sort_key)]
    id: String,

    username: String,
    email_address: String,
    first_name: String,
    last_name: String,
}

#[derive(Default, Clone, Serialize, dynomite::Item)]
pub struct News {
    #[serde(skip)]
    #[dynomite(partition_key)]
    #[dynomite(rename = "type")]
    r#type: String,

    #[dynomite(sort_key)]
    id: String,

    title: String,
    timestamp: i64,
    summary: String,
    author: String,
    news: String,
}
