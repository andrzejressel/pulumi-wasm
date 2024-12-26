#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptAnalyticsEngineBinding {
    /// The name of the Analytics Engine dataset to write to.
    #[builder(into)]
    #[serde(rename = "dataset")]
    pub r#dataset: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
