#[derive(serde::Serialize)]
pub struct WorkerScriptWebassemblyBinding {
    /// The base64 encoded wasm module you want to store.
    #[serde(rename = "module")]
    pub r#module: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
