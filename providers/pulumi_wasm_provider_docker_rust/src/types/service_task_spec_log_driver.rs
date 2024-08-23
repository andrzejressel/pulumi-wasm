#[derive(serde::Serialize)]
pub struct ServiceTaskSpecLogDriver {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "options")]
    pub r#options: Box<Option<std::collections::HashMap<String, String>>>,
}
