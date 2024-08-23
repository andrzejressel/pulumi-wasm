#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacement {
    #[serde(rename = "constraints")]
    pub r#constraints: Box<Option<Vec<String>>>,
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<Option<i32>>,
    #[serde(rename = "platforms")]
    pub r#platforms: Box<Option<Vec<crate::types::ServiceTaskSpecPlacementPlatform>>>,
    #[serde(rename = "prefs")]
    pub r#prefs: Box<Option<Vec<String>>>,
}
