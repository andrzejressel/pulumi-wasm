#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecLogDriver {
    /// The logging driver to use
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The options for the logging driver
    #[serde(rename = "options")]
    pub r#options: Box<Option<std::collections::HashMap<String, String>>>,
}
