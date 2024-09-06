#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WorkerScriptPlacement {
    /// The placement mode for the Worker. Available values: `smart`.
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
