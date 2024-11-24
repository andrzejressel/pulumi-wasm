#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptSecretTextBinding {
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The secret text you want to store.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
