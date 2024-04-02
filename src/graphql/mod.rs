pub mod model;
mod ql;

mod graphiql_v2_source_docs;
mod schema;

use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::Response;
use async_graphql::{dataloader::DataLoader, EmptySubscription};
use async_graphql::{Schema, ServerError};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use schema::{ChatSchema, MutationRoot, QueryRoot};

#[allow(dead_code)]
#[derive(Clone)]
pub struct GraphQl {
    schema: ChatSchema,
}

impl GraphQl {
    pub fn new() -> Self {
        // create and initialize graphql schema
        let schema = Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription::default(),
        )
        .finish();

        Self { schema }
    }
}

const API_URL: &str = "/";

async fn index(schema: web::Data<ChatSchema>, req: GraphQLRequest) -> GraphQLResponse {
    let req = req.into_inner();
    // insert subject into context, so we can have auth0 user id and email on each graphql request
    // req.data.insert(sub);
    schema.execute(req).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            graphiql_v2_source_docs::GraphiQLSource::build()
                .endpoint(API_URL)
                .subscription_endpoint(API_URL)
                .include_query(true)
                .finish(),
        ))
}

async fn index_ws(
    schema: web::Data<ChatSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    // insert subject into context, so we can have auth0 user id and email on each graphql request
    let data = async_graphql::context::Data::default();
    // data.insert(sub);

    let schema = Schema::clone(&schema);
    let subscription = GraphQLSubscription::new(schema).with_data(data);
    subscription.start(&req, payload)
}

impl GraphQl {
    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        // initialize graphql schema
        cfg.app_data(web::Data::new(self.schema.clone()));

        // graphql endpoint
        cfg.service(web::resource(API_URL).guard(guard::Post()).to(index));

        // graphql subscription endpoint
        cfg.service(
            web::resource(API_URL)
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        );

        cfg.service(
            web::resource(API_URL)
                .guard(guard::Get())
                .to(index_graphiql),
        );
    }
}
