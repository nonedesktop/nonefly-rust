use crate::storage;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use sqlx::SqlitePool;

use anyhow::Result;

#[derive(Deserialize, Serialize)]
pub struct Adapter {
    #[serde(rename(deserialize = "project_link"))]
    pub python_package_name: String,
    pub module_name: String,
    #[serde(skip_deserializing)]
    pub data_json: Option<String>,
}

#[derive(Deserialize)]
pub struct Plugin {
    #[serde(rename(deserialize = "project_link"))]
    pub python_package_name: String,
    pub module_name: String,
    #[serde(skip_deserializing)]
    pub data_json: Option<String>,
}

pub async fn update_adapter_index(pool: &SqlitePool) -> Result<()> {
    let adapters = reqwest::get("https://registry.nonebot.dev/adapters.json")
        .await?
        .json::<Vec<Box<RawValue>>>()
        .await?
        .into_iter()
        .map(|adapter_json| -> Result<_> {
            let mut adapter: Adapter = serde_json::from_str(adapter_json.get())?;
            adapter.data_json = Some(adapter_json.get().to_string());

            Ok(adapter)
        })
        .collect::<Result<_>>()?;
    storage::save_adapters(pool, &adapters).await?;

    Ok(())
}

pub async fn update_plugin_index(pool: &SqlitePool) -> Result<()> {
    let plugins = reqwest::get("https://registry.nonebot.dev/plugins.json")
        .await?
        .json::<Vec<Box<RawValue>>>()
        .await?
        .into_iter()
        .map(|plugin_json| -> Result<_> {
            let mut plugin: Plugin = serde_json::from_str(plugin_json.get())?;
            plugin.data_json = Some(plugin_json.get().to_string());

            Ok(plugin)
        })
        .collect::<Result<_>>()?;
    storage::save_plugins(pool, &plugins).await?;

    Ok(())
}

pub async fn get_adapters(pool: &SqlitePool) -> Result<Vec<String>> {
    storage::load_adapters(pool).await
}
