#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WorkerScriptPlainTextBinding {
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The plain text you want to store.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
