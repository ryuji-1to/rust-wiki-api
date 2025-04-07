use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use model::{mutation::Mutation, query::Query, WikiSchema};
use rocket::{response::content, routes, State};
use sqlx::postgres::PgPoolOptions;
mod db;
mod model;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<WikiSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<WikiSchema>, request: GraphQLRequest) -> GraphQLResponse {
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
