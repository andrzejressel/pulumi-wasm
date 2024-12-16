#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceModeReplicated {
    /// The amount of replicas of the service. Defaults to `1`
    #[builder(into, default)]
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Option<i32>>,
}
