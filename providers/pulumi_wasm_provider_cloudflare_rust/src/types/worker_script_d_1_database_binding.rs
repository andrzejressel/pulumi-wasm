#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WorkerScriptD1DatabaseBinding {
    /// Database ID of D1 database to use.
    #[serde(rename = "databaseId")]
    pub r#database_id: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
