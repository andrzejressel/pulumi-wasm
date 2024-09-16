#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceModeReplicated {
    /// The amount of replicas of the service. Defaults to `1`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Option<i32>>,
}
