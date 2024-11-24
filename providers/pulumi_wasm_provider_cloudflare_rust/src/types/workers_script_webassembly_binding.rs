#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptWebassemblyBinding {
    /// The base64 encoded wasm module you want to store.
    #[builder(into)]
    #[serde(rename = "module")]
    pub r#module: Box<String>,
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
