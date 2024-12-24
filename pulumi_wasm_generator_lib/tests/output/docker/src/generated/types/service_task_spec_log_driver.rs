#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecLogDriver {
    /// The logging driver to use
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The options for the logging driver
    #[builder(into, default)]
    #[serde(rename = "options")]
    pub r#options: Box<Option<std::collections::HashMap<String, String>>>,
}
