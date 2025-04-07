use async_graphql::Object;
use sqlx::PgPool;

use crate::db::PageRecord;

use super::Page;

pub struct Query;

#[Object]
impl Query {
    async fn answer(&self, ctx: &async_graphql::Context<'_>) -> Result<i32, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let a: (i32,) = sqlx::query_as("select 42;").fetch_one(pool).await?;
        Ok(a.0)
    }

    async fn page(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> Result<Option<Page>, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let page_record: Option<PageRecord> = sqlx::query_as(
            "select id, title, body, create_time, update_time from pages where id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        let page = page_record.map(Into::into);
        Ok(page)
    }
}
