#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptPlainTextBinding {
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The plain text you want to store.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
