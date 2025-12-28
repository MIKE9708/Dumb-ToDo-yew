use axum::{Json, Router, extract::State, response::IntoResponse, routing::get, routing::post};
use common::ToDo;
use deadpool_redis::{Config, Pool, Runtime, redis};
use log::{error, info};
use serde_json;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

const REDIS_CONN: &'static str = "redis://127.0.0.1";
const SERVER_CONN: &'static str = "127.0.0.1:3000";

const UNABLE_TO_CONNECT: &'static str = "Unable to connect to Redis";
const FAILED_TO_STORE_DATA: &'static str = "Failed to store data";
const FAILED_TO_DELETE_DATA: &'static str = "Failed to delete data";
const FAILED_TO_RETRIEVE_DATA: &'static str = "Failed to retireve data";

fn get_redis_conn() -> Result<Pool, &'static str> {
    let cfg = Config::from_url(REDIS_CONN);
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .map_err(|_| UNABLE_TO_CONNECT)?;

    Ok(pool)
}
async fn not_found() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Route not found")
}

async fn store_todo(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<ToDo>,
) -> Result<(), &'static str> {
    let mut conn = pool.get().await.map_err(|_| UNABLE_TO_CONNECT)?;
    let json = serde_json::to_string(&payload).map_err(|_| common::UNABLE_TO_PARSE_DATA)?;

    let _: () = redis::cmd("SET")
        .arg(&payload.id)
        .arg(&json)
        .query_async(&mut conn)
        .await
        .map_err(|e| {
            error!("{}: {}", FAILED_TO_STORE_DATA, e);
            FAILED_TO_STORE_DATA
        })?;

    Ok(())
}

async fn delete_todo(
    State(pool): State<Arc<Pool>>,
    Json(payload): Json<ToDo>,
) -> Result<(), &'static str> {
    let mut conn = pool.get().await.map_err(|_| UNABLE_TO_CONNECT)?;
    let _: () = redis::cmd("DEL")
        .arg(&payload.id)
        .query_async(&mut conn)
        .await
        .map_err(|e| {
            error!("{}: {:?}", FAILED_TO_DELETE_DATA, e);
            return FAILED_TO_DELETE_DATA;
        })?;

    info!("Deleted `Data: {:?}", payload);

    Ok(())
}

async fn get_todo(State(pool): State<Arc<Pool>>) -> Result<Json<Vec<ToDo>>, &'static str> {
    let mut conn = pool.get().await.map_err(|_| UNABLE_TO_CONNECT)?;

    let mut cursor: u64 = 0;
    let mut todo_vec: Vec<ToDo> = vec![];

    info!("GetTodo called");
    loop {
        let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
            .cursor_arg(cursor)
            .query_async(&mut conn)
            .await
            .map_err(|e| {
                error!("{}: {}", FAILED_TO_RETRIEVE_DATA, e);
                FAILED_TO_RETRIEVE_DATA
            })?;

        for key in keys {
            let todo_str: String = redis::cmd("GET")
                .arg(key)
                .query_async(&mut conn)
                .await
                .map_err(|e| {
                    error!("{}: {}", FAILED_TO_RETRIEVE_DATA, e);
                    FAILED_TO_STORE_DATA
                })?;
            // BAD BAD BAD
            let todo: ToDo = serde_json::from_str(&todo_str).unwrap();
            todo_vec.push(todo);
        }
        cursor = new_cursor;

        if cursor == 0 {
            break;
        }
    }

    info!("Retrieved Data: {:?}", todo_vec);
    Ok(Json(todo_vec))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .init();
    let redis_conn = get_redis_conn().unwrap();
    let app = Router::new()
        .route("/store_todo", post(store_todo))
        .route("/delete_todo", post(delete_todo))
        .route("/get_todo", get(get_todo))
        .fallback(not_found)
        .with_state(Arc::new(redis_conn))
        .layer(TraceLayer::new_for_http());

    info!("Starting Simple Server on: {:?}", SERVER_CONN);
    let listener = tokio::net::TcpListener::bind(SERVER_CONN).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
