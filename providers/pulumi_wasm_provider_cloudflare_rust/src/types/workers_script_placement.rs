#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct WorkersScriptPlacement {
    /// The placement mode for the Worker. Available values: `smart`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
