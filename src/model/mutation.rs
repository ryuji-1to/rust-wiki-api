use async_graphql::InputObject;
use async_graphql::Object;
use sqlx::PgPool;

use crate::db::PageRecord;

use super::Page;

pub struct Mutation;
#[derive(InputObject)]

struct CreatePageInput {
    title: String,
    body: String,
}

#[Object]
impl Mutation {
    async fn create_page(
        &self,
        ctx: &async_graphql::Context<'_>,
        input: CreatePageInput,
    ) -> Result<Page, async_graphql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let sql = "
                insert into pages ( 
                        title, body, create_time, update_time 
                    ) values ( 
                        $1, $2, current_timestamp, current_timestamp 
                    )
                    returning 
                        id, title,body, create_time, update_time
                ;";
        let page_record: PageRecord = sqlx::query_as(sql)
            .bind(&input.title)
            .bind(&input.body)
            .fetch_one(&mut *tx)
            .await?;
        let sql = "
        insert into page_revisions (
            page_id, body, author, create_time
        )
        values (
            $1, $2, $3, $4
        );
        ";
        sqlx::query(sql)
            .bind(page_record.id)
            .bind(&input.body)
            .bind("hoge@hoge.com")
            .bind(page_record.create_time)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        let gql_page = page_record.into();
        Ok(gql_page)
    }
}
