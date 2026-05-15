//! Main entry point for the dm-graphql-api GraphQL server

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQlResponse};
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create GraphQL schema (this would be replaced with your actual schema)
    let schema = async_graphql::Schema::build()
        .finish()
        .expect("Failed to build schema");

    // Create the Axum router
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/playground", get(graphql_playground))
        .with_state(schema);

    // Bind to address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Listening on {}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn graphql_handler(
    schema: axum::extract::State<async_graphql::Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new().endpoint("/graphql"),
    ))
}
