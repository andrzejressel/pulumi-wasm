#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WorkerScriptAnalyticsEngineBinding {
    /// The name of the Analytics Engine dataset to write to.
    #[serde(rename = "dataset")]
    pub r#dataset: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
