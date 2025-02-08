#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameter {
    /// Any of the host devices to expose to the container.
    #[builder(into)]
    #[serde(rename = "devices")]
    pub r#devices: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameterDevice>>,
    /// If true, run an init process inside the container that forwards signals and reaps processes.
    #[builder(into)]
    #[serde(rename = "initProcessEnabled")]
    pub r#init_process_enabled: Box<bool>,
    /// The total amount of swap memory (in MiB) a container can use.
    #[builder(into)]
    #[serde(rename = "maxSwap")]
    pub r#max_swap: Box<i32>,
    /// The value for the size (in MiB) of the `/dev/shm` volume.
    #[builder(into)]
    #[serde(rename = "sharedMemorySize")]
    pub r#shared_memory_size: Box<i32>,
    /// You can use this parameter to tune a container's memory swappiness behavior.
    #[builder(into)]
    #[serde(rename = "swappiness")]
    pub r#swappiness: Box<i32>,
    /// The container path, mount options, and size (in MiB) of the tmpfs mount.
    #[builder(into)]
    #[serde(rename = "tmpfs")]
    pub r#tmpfs: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameterTmpf>>,
}
