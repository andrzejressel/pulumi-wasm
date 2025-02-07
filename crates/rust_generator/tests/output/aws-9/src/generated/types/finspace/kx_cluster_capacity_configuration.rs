#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxClusterCapacityConfiguration {
    /// Number of instances running in a cluster. Must be at least 1 and at most 5.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    /// Determines the hardware of the host computer used for your cluster instance. Each node type offers different memory and storage capabilities. Choose a node type based on the requirements of the application or software that you plan to run on your instance.
    /// 
    /// You can only specify one of the following values:
    /// * kx.s.large – The node type with a configuration of 12 GiB memory and 2 vCPUs.
    /// * kx.s.xlarge – The node type with a configuration of 27 GiB memory and 4 vCPUs.
    /// * kx.s.2xlarge – The node type with a configuration of 54 GiB memory and 8 vCPUs.
    /// * kx.s.4xlarge – The node type with a configuration of 108 GiB memory and 16 vCPUs.
    /// * kx.s.8xlarge – The node type with a configuration of 216 GiB memory and 32 vCPUs.
    /// * kx.s.16xlarge – The node type with a configuration of 432 GiB memory and 64 vCPUs.
    /// * kx.s.32xlarge – The node type with a configuration of 864 GiB memory and 128 vCPUs.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Box<String>,
}
