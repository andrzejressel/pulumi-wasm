#[derive(serde::Serialize)]
pub struct ApiTokenPolicy {
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    #[serde(rename = "permissionGroups")]
    pub r#permission_groups: Box<Vec<String>>,
    #[serde(rename = "resources")]
    pub r#resources: Box<std::collections::HashMap<String, String>>,
}
