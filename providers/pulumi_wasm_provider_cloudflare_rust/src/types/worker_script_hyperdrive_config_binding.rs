#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WorkerScriptHyperdriveConfigBinding {
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    /// The ID of the Hyperdrive config to use.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
