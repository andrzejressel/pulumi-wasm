#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTaskSpec {
    /// The spec for each container
    #[builder(into)]
    #[serde(rename = "containerSpec")]
    pub r#container_spec: Box<super::types::ServiceTaskSpecContainerSpec>,
    /// A counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126).
    #[builder(into, default)]
    #[serde(rename = "forceUpdate")]
    pub r#force_update: Box<Option<i32>>,
    /// Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified
    #[builder(into, default)]
    #[serde(rename = "logDriver")]
    pub r#log_driver: Box<Option<super::types::ServiceTaskSpecLogDriver>>,
    /// The networks the container is attached to
    #[builder(into, default)]
    #[serde(rename = "networksAdvanceds")]
    pub r#networks_advanceds: Box<Option<Vec<super::types::ServiceTaskSpecNetworksAdvanced>>>,
    /// The placement preferences
    #[builder(into, default)]
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<super::types::ServiceTaskSpecPlacement>>,
    /// Resource requirements which apply to each individual container created as part of the service
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<super::types::ServiceTaskSpecResources>>,
    /// Specification for the restart policy which applies to containers created as part of this service.
    #[builder(into, default)]
    #[serde(rename = "restartPolicy")]
    pub r#restart_policy: Box<Option<super::types::ServiceTaskSpecRestartPolicy>>,
    /// Runtime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go).
    #[builder(into, default)]
    #[serde(rename = "runtime")]
    pub r#runtime: Box<Option<String>>,
}
