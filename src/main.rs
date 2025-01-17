use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use axum::{routing::get, routing::post, Router};
use config::Config;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};

async fn hello_world() -> &'static str {
    "Hello world!"
}

async fn create_people(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Json<String> {
    let _ = sqlx::query(
        "INSERT INTO people (name, age) VALUES ($1, 13)"
    )
    .bind(&name)
    .execute(&state.db)
    .await.unwrap();

    Json(format!("{} created", name))
}

async fn list_people(State(state): State<Arc<AppState>>) -> Json<Vec<String>> {
    let result = sqlx::query(
        "SELECT name FROM people"
    )
    .fetch_all(&state.db)
    .await.unwrap();

    Json(result.into_iter()
        .map(|row| row.get(0))
        .collect())
}

#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
struct DbConfig {
    user: String,
    password: String,
    host: String,
    port: String,
    name: String,
}

impl DbConfig {
    pub fn connection_url(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.name)
    }
}

#[derive(Debug, Default, serde::Deserialize, PartialEq, Eq)]
struct AppConfig {
    database: DbConfig
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&app.database.connection_url())
        .await
        .unwrap();

    let state = Arc::new(AppState { db: pool });

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/people/{name}", post(create_people))
        .route("/people", get(list_people))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
