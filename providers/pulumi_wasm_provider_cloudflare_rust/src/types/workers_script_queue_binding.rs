#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptQueueBinding {
    /// The name of the global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    /// Name of the queue you want to use.
    #[builder(into)]
    #[serde(rename = "queue")]
    pub r#queue: Box<String>,
}