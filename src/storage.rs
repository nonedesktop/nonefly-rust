use crate::{
    instance::Instance,
    nonebot::{Adapter, Plugin},
};

use sqlx::{query, types::Json, SqlitePool};

use serde_json::value::RawValue;

use anyhow::Result;

pub async fn save_instance(pool: &SqlitePool, name: &String, instance: &Instance) -> Result<i64> {
    let instance_json = Json::from(instance);

    Ok(query!(
        r#"INSERT INTO instance (name, instance_json) VALUES ($1, $2) RETURNING id"#,
        name,
        instance_json,
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn load_instance(pool: &SqlitePool, id: &i64) -> Result<Option<Instance>> {
    Ok(query!(
        r#"SELECT instance_json AS "instance_json: Json<Instance>" FROM instance WHERE id = $1"#,
        id,
    )
    .fetch_optional(pool)
    .await?
    .map(|result| result.instance_json.0))
}

pub async fn save_adapters(pool: &SqlitePool, adapters: Vec<Adapter>) -> Result<()> {
    let mut transaction = pool.begin().await?;

    query!(r#"DELETE FROM adapter"#)
        .execute(&mut *transaction)
        .await?;
    for adapter in adapters {
        let data_json = adapter.data_json.unwrap().get().to_string();
        query!(
            r#"INSERT INTO adapter (module_name, python_package_name, data_json) VALUES ($1, $2, $3)"#,
            adapter.module_name,
            adapter.python_package_name,
            data_json,
        ).execute(&mut *transaction).await?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn load_adapters(pool: &SqlitePool) -> Result<Vec<Box<RawValue>>> {
    query!(r#"SELECT data_json FROM adapter"#)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|result| Ok(serde_json::from_str(&result.data_json)?))
        .collect::<Result<_>>()
}

pub async fn save_plugins(pool: &SqlitePool, plugins: Vec<Plugin>) -> Result<()> {
    let mut transaction = pool.begin().await?;

    query!(r#"DELETE FROM plugin"#)
        .execute(&mut *transaction)
        .await?;
    for plugin in plugins {
        let data_json = plugin.data_json.unwrap().get().to_string();
        query!(
            r#"INSERT INTO plugin (module_name, python_package_name, data_json) VALUES ($1, $2, $3)"#,
            plugin.module_name,
            plugin.python_package_name,
            data_json,
        ).execute(&mut *transaction).await?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn load_plugins(pool: &SqlitePool) -> Result<Vec<Box<RawValue>>> {
    query!(r#"SELECT data_json FROM plugin"#)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|result| Ok(serde_json::from_str(&result.data_json)?))
        .collect::<Result<_>>()
}
