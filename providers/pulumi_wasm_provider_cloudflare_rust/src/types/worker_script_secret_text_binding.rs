#[derive(serde::Serialize)]
pub struct WorkerScriptSecretTextBinding {
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The secret text you want to store.
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
