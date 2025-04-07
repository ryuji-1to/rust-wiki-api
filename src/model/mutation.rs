use async_graphql::InputObject;
use async_graphql::Object;
use pulldown_cmark::{Options, Parser};
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
        let gql_page = Page {
            id: page_record.id,
            title: page_record.title,
            body_html: page_record.body,
        };
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
        let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(markdown_input, options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        let expected_html =
            "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
        Ok(gql_page)
    }
}
