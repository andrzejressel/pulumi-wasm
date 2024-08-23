#[derive(serde::Serialize)]
pub struct ServiceTaskSpec {
    #[serde(rename = "containerSpec")]
    pub r#container_spec: Box<crate::types::ServiceTaskSpecContainerSpec>,
    #[serde(rename = "forceUpdate")]
    pub r#force_update: Box<Option<i32>>,
    #[serde(rename = "logDriver")]
    pub r#log_driver: Box<Option<crate::types::ServiceTaskSpecLogDriver>>,
    #[serde(rename = "networksAdvanceds")]
    pub r#networks_advanceds: Box<Option<Vec<crate::types::ServiceTaskSpecNetworksAdvanced>>>,
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::ServiceTaskSpecPlacement>>,
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<crate::types::ServiceTaskSpecResources>>,
    #[serde(rename = "restartPolicy")]
    pub r#restart_policy: Box<Option<crate::types::ServiceTaskSpecRestartPolicy>>,
    #[serde(rename = "runtime")]
    pub r#runtime: Box<Option<String>>,
}
