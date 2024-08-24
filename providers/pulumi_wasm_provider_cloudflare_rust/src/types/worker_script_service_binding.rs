#[derive(serde::Serialize)]
pub struct WorkerScriptServiceBinding {
    /// The name of the Worker environment to bind to.
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the Worker to bind to.
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
