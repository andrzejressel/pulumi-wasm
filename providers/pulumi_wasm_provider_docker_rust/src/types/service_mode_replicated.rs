#[derive(serde::Serialize)]
pub struct ServiceModeReplicated {
    /// The amount of replicas of the service. Defaults to `1`
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Option<i32>>,
}
