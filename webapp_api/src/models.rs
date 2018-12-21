use chrono::offset::Utc;
use chrono::DateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    body: String,
    author: String,
    datetime: DateTime<Utc>,
    uuid: Uuid,
}

impl Post {
    pub fn new(title: &str, body: &str, author: &str, datetime: DateTime<Utc>, uuid: Uuid) -> Post {
        Post {
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            datetime,
            uuid,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}
