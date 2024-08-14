use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct NewsAuthor {
    id: String,

    username: String,
    email_address: String,
    first_name: String,
    last_name: String,
}

impl From<DbNewsAuthor> for NewsAuthor {
    fn from(db: DbNewsAuthor) -> Self {
        db.news_author
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DbNewsAuthor {
    #[serde(rename = "type")]
    r#type: String,

    #[serde(flatten)]
    news_author: NewsAuthor,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct News {
    id: String,

    title: String,
    timestamp: i64,
    summary: String,
    author: String,
    news: String,
}

impl From<DbNews> for News {
    fn from(db: DbNews) -> Self {
        db.news
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DbNews {
    #[serde(rename = "type")]
    r#type: String,

    #[serde(flatten)]
    news: News,
}
