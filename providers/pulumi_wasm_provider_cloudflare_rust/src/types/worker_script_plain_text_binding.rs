#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WorkerScriptPlainTextBinding {
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The plain text you want to store.
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
