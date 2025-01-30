use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use axum::{routing::get, routing::post, Router};
use config::Config;
use persistence::postgres::Pg;
use persistence::{DbConfig, PersonRepo};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};

pub mod persistence;
pub mod model;

async fn hello_world() -> &'static str {
    "Hello world!"
}
//
//async fn create_people(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Json<String> {
//    let _ = sqlx::query(
//        "INSERT INTO people (name, age) VALUES ($1, 13)"
//    )
//    .bind(&name)
//    .execute(&state.db)
//    .await.unwrap();
//
//    Json(format!("{} created", name))
//}
//
//async fn list_people(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
//    let result = sqlx::query(
//        "SELECT name FROM people"
//    )
//    .fetch_all(&state.db)
//    .await.unwrap();
//
//    Json(result.into_iter()
//        .map(|row| row.get(0))
//        .collect())
//}

#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
struct AppConfig {
    database: DbConfig
}

#[derive(Clone)]
struct AppState<PR: PersonRepo> {
    person_repo: PR
}

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
        )
        .build()
        .unwrap();

    let app: AppConfig = config.try_deserialize().unwrap();
    println!("{:?}", app);

    let pg = Pg::new(&app.database.connection_url()).await;

    let state = Arc::new(AppState { person_repo: pg });

    let router = Router::new()
        .route("/", get(hello_world))
        //.route("/people/{name}", post(create_people))
        //.route("/people", get(list_people))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
