#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptD1DatabaseBinding {
    /// Database ID of D1 database to use.
    #[builder(into)]
    #[serde(rename = "databaseId")]
    pub r#database_id: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
