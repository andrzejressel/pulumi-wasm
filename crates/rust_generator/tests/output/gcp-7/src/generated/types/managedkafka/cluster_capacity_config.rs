#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterCapacityConfig {
    /// The memory to provision for the cluster in bytes. The value must be between 1 GiB and 8 GiB per vCPU. Ex. 1024Mi, 4Gi.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<String>,
    /// The number of vCPUs to provision for the cluster. The minimum is 3.
    #[builder(into)]
    #[serde(rename = "vcpuCount")]
    pub r#vcpu_count: Box<String>,
}
