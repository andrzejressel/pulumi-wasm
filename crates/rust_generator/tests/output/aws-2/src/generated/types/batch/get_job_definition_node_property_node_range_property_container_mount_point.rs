#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerMountPoint {
    /// The absolute file path in the container where the tmpfs volume is mounted.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<String>,
    /// If this value is true, the container has read-only access to the volume.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<bool>,
    /// The name of the volume to mount.
    #[builder(into)]
    #[serde(rename = "sourceVolume")]
    pub r#source_volume: Box<String>,
}
