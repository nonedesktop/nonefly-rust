mod instance;
mod nonebot;
mod storage;

use instance::Instance;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json,
};

use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    Sqlite, SqlitePool,
};

use serde::Deserialize;
use serde_json::value::RawValue;

const MIGRATOR: Migrator = sqlx::migrate!();
const DATABASE_FILENAME: &str = "nonefly.db";

#[derive(Clone)]
struct AppState {
    sqlite_pool: SqlitePool,
}

struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

async fn update_adapter_index_handler(State(state): State<AppState>) -> Result<(), Error> {
    nonebot::update_adapter_index(&state.sqlite_pool).await?;

    Ok(())
}

async fn update_plugin_index_handler(State(state): State<AppState>) -> Result<(), Error> {
    nonebot::update_plugin_index(&state.sqlite_pool).await?;

    Ok(())
}

async fn get_adapters_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Box<RawValue>>>, Error> {
    Ok(Json(nonebot::get_adapters(&state.sqlite_pool).await?))
}

async fn get_plugins_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Box<RawValue>>>, Error> {
    Ok(Json(nonebot::get_plugins(&state.sqlite_pool).await?))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateInstanceInput {
    name: String,
    working_directory: String,
}

async fn create_instance_handler(
    State(state): State<AppState>,
    extract::Json(input): extract::Json<CreateInstanceInput>,
) -> Result<Json<i64>, Error> {
    let instance = Instance::new(input.working_directory)?;
    instance.create()?;

    Ok(Json(
        storage::save_instance(&state.sqlite_pool, &input.name, &instance).await?,
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartInstanceInput {
    id: i64,
}

async fn start_instance_handler(
    State(state): State<AppState>,
    extract::Json(input): extract::Json<StartInstanceInput>,
) -> Result<(), Error> {
    let instance = storage::load_instance(&state.sqlite_pool, &input.id).await?;
    if instance.is_none() {
        return Err(anyhow::anyhow!("Instance does not exist").into());
    }
    let instance = instance.unwrap();

    Ok(instance.start()?)
}

async fn guard_sqlite_pool() -> anyhow::Result<SqlitePool> {
    if !Sqlite::database_exists(DATABASE_FILENAME).await? {
        Sqlite::create_database(DATABASE_FILENAME).await?;
    }

    let pool = SqlitePool::connect(DATABASE_FILENAME).await?;
    MIGRATOR.run(&pool).await?;

    Ok(pool)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        sqlite_pool: guard_sqlite_pool().await.unwrap(),
    };

    let router = axum::Router::new()
        .route("/update-adapter-index", get(update_adapter_index_handler))
        .route("/update-plugin-index", get(update_plugin_index_handler))
        .route("/get-adapters", get(get_adapters_handler))
        .route("/get-plugins", get(get_plugins_handler))
        .route("/instance/create", post(create_instance_handler))
        .route("/instance/start", post(start_instance_handler))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
