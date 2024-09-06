#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WorkerScriptQueueBinding {
    /// The name of the global variable for the binding in your Worker code.
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    /// Name of the queue you want to use.
    #[serde(rename = "queue")]
    pub r#queue: Box<String>,
}
