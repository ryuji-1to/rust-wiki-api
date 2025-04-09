use async_graphql::{EmptySubscription, Object, Schema};
use chrono::NaiveDateTime;
use mutation::Mutation;
use pulldown_cmark::{Options, Parser};
use query::Query;

use crate::db::PageRecord;

pub mod mutation;
pub mod query;

pub type WikiSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Debug)]
pub struct Page {
    id: i32,
    title: String,
    body: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

impl From<PageRecord> for Page {
    fn from(
        PageRecord {
            id,
            title,
            body,
            create_time,
            update_time,
        }: PageRecord,
    ) -> Self {
        Self {
            id,
            title,
            body,
            create_time,
            update_time,
        }
    }
}

#[Object]
impl Page {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn title(&self) -> &str {
        &self.title
    }
    async fn body_html(&self) -> Result<String, async_graphql::Error> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&self.body, options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        Ok(html_output)
    }
    async fn create_time(&self) -> String {
        self.create_time.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
