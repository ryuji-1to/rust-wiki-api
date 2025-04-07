use async_graphql::Object;
use sqlx::PgPool;

pub struct Query;

#[Object]
impl Query {
    async fn answer(&self, ctx: &async_graphql::Context<'_>) -> Result<i32, async_graphql::Error> {
        let pool = ctx
            .data::<PgPool>()
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;
        let a: (i32,) = sqlx::query_as("select 42;").fetch_one(pool).await?;
        Ok(a.0)
    }

    async fn hey(&self) -> String {
        "hello".into()
    }
}
