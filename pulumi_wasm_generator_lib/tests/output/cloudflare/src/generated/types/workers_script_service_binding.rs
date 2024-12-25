#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptServiceBinding {
    /// The name of the Worker environment to bind to.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the Worker to bind to.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
