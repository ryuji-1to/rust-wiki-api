use async_graphql::{self as agql, InputObject, SimpleObject};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, routes, State};
use sqlx::prelude::FromRow;
use sqlx::{postgres::PgPoolOptions, PgPool};

struct Query;

#[Object]
impl Query {
    async fn answer(&self, ctx: &async_graphql::Context<'_>) -> Result<i32, agql::Error> {
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

#[derive(SimpleObject)]
struct Page {
    id: i32,
    title: String,
    body_html: String,
}

#[derive(Debug, FromRow)]
struct PageRecord {
    id: i32,
    title: String,
    body: String,
    create_time: chrono::NaiveDateTime,
    update_time: chrono::NaiveDateTime,
}

#[derive(InputObject)]
struct CreatePageInput {
    title: String,
    body: String,
}

struct Mutation;

#[Object]
impl Mutation {
    async fn create_page(
        &self,
        ctx: &agql::Context<'_>,
        input: CreatePageInput,
    ) -> Result<Page, agql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let sql = "insert into pages ( 
                                title, body, create_time, update_time 
                            ) values ( 
                                $1, $2, current_timestamp, current_timestamp 
                            )
                            returning 
                                id, title,body, create_time, update_time;";
        let page_record: PageRecord = sqlx::query_as(sql)
            .bind(input.title)
            .bind(input.body)
            .fetch_one(&mut *tx)
            .await?;
        let gql_page = Page {
            id: page_record.id,
            title: page_record.title,
            body_html: page_record.body,
        };
        tx.commit().await?;
        Ok(gql_page)
    }
}

type SomeSchema = Schema<Query, Mutation, EmptySubscription>;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<SomeSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SomeSchema>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[rocket::launch]
async fn rocket() -> _ {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres:///wiki_dev")
        .await
        .unwrap();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();
    rocket::build()
        .manage(schema)
        .mount("/", routes![graphiql, graphql_query, graphql_request])
}
