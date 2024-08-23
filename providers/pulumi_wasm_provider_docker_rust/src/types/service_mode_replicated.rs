#[derive(serde::Serialize)]
pub struct ServiceModeReplicated {
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Option<i32>>,
}
