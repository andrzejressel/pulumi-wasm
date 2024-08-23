#[derive(serde::Serialize)]
pub struct WorkerScriptD1DatabaseBinding {
    #[serde(rename = "databaseId")]
    pub r#database_id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
