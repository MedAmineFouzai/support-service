mod context;
mod models;
mod queue;
mod services;
mod utils;
use actix_cors::Cors;
use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use services::SupportSchema;
use utils::GenerateSDL;

use crate::{
    context::{AppState, CollectionContainer},
    models::SupportCollection,
    services::{MutationRoot, QueryRoot, Storage, SubscriptionRoot},
};

async fn index(schema: web::Data<SupportSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/")
                .finish(),
        ))
}

async fn index_ws(
    schema: web::Data<SupportSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let support_collection = SupportCollection::connect().await;
    let collection_container =
        CollectionContainer::new(SupportCollection::new(support_collection.clone()));

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .data(AppState {
            container: collection_container,
        })
        .finish();
    GenerateSDL::export(schema.sdl());
    println!("GraphiQL IDE: http://localhost:5002");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:5002")?
    .run()
    .await
}
