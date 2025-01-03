#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterNetworking {
    /// Disable the per node pool subnet security group rules on the control plane security group. When set to true, you must also provide one or more security groups that ensure node pools are able to send requests to the control plane on TCP/443 and TCP/8132. Failure to do so may result in unavailable node pools.
    #[builder(into, default)]
    #[serde(rename = "perNodePoolSgRulesDisabled")]
    pub r#per_node_pool_sg_rules_disabled: Box<Option<bool>>,
    /// All pods in the cluster are assigned an RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation.
    #[builder(into)]
    #[serde(rename = "podAddressCidrBlocks")]
    pub r#pod_address_cidr_blocks: Box<Vec<String>>,
    /// All services in the cluster are assigned an RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation.
    #[builder(into)]
    #[serde(rename = "serviceAddressCidrBlocks")]
    pub r#service_address_cidr_blocks: Box<Vec<String>>,
    /// The VPC associated with the cluster. All component clusters (i.e. control plane and node pools) run on a single VPC. This field cannot be changed after creation.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
