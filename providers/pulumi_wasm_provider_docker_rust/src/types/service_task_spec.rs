#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpec {
    /// The spec for each container
    #[builder(into)]
    #[serde(rename = "containerSpec")]
    pub r#container_spec: Box<crate::types::ServiceTaskSpecContainerSpec>,
    /// A counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126).
    #[builder(into, default)]
    #[serde(rename = "forceUpdate")]
    pub r#force_update: Box<Option<i32>>,
    /// Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified
    #[builder(into, default)]
    #[serde(rename = "logDriver")]
    pub r#log_driver: Box<Option<crate::types::ServiceTaskSpecLogDriver>>,
    /// The networks the container is attached to
    #[builder(into, default)]
    #[serde(rename = "networksAdvanceds")]
    pub r#networks_advanceds: Box<Option<Vec<crate::types::ServiceTaskSpecNetworksAdvanced>>>,
    /// The placement preferences
    #[builder(into, default)]
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::ServiceTaskSpecPlacement>>,
    /// Resource requirements which apply to each individual container created as part of the service
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<crate::types::ServiceTaskSpecResources>>,
    /// Specification for the restart policy which applies to containers created as part of this service.
    #[builder(into, default)]
    #[serde(rename = "restartPolicy")]
    pub r#restart_policy: Box<Option<crate::types::ServiceTaskSpecRestartPolicy>>,
    /// Runtime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go).
    #[builder(into, default)]
    #[serde(rename = "runtime")]
    pub r#runtime: Box<Option<String>>,
}
