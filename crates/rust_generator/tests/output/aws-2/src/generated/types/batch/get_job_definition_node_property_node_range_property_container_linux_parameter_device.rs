#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameterDevice {
    /// The absolute file path in the container where the tmpfs volume is mounted.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<String>,
    /// The path for the device on the host container instance.
    #[builder(into)]
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<String>,
    /// The explicit permissions to provide to the container for the device.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Vec<String>>,
}
