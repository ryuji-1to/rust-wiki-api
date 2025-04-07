use async_graphql::{EmptySubscription, Schema, SimpleObject};
use mutation::Mutation;
use query::Query;

pub mod mutation;
pub mod query;

pub type WikiSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(SimpleObject)]
pub struct Page {
    id: i32,
    title: String,
    body_html: String,
}
