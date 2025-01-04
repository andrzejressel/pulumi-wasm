#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpcIpamPoolsIpamPool {
    /// IP protocol assigned to this pool.
    #[builder(into)]
    #[serde(rename = "addressFamily")]
    pub r#address_family: Box<String>,
    /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is `10.0.0.0/8` and you enter 16 here, new allocations will default to `10.0.0.0/16`.
    #[builder(into)]
    #[serde(rename = "allocationDefaultNetmaskLength")]
    pub r#allocation_default_netmask_length: Box<i32>,
    /// The maximum netmask length that will be required for CIDR allocations in this pool.
    #[builder(into)]
    #[serde(rename = "allocationMaxNetmaskLength")]
    pub r#allocation_max_netmask_length: Box<i32>,
    /// The minimum netmask length that will be required for CIDR allocations in this pool.
    #[builder(into)]
    #[serde(rename = "allocationMinNetmaskLength")]
    pub r#allocation_min_netmask_length: Box<i32>,
    /// Tags that are required to create resources in using this pool.
    #[builder(into)]
    #[serde(rename = "allocationResourceTags")]
    pub r#allocation_resource_tags: Box<std::collections::HashMap<String, String>>,
    /// ARN of the pool
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// If enabled, IPAM will continuously look for resources within the CIDR range of this pool and automatically import them as allocations into your IPAM.
    #[builder(into)]
    #[serde(rename = "autoImport")]
    pub r#auto_import: Box<bool>,
    /// Limits which service in AWS that the pool can be used in. `ec2` for example, allows users to use space for Elastic IP addresses and VPCs.
    #[builder(into)]
    #[serde(rename = "awsService")]
    pub r#aws_service: Box<String>,
    /// Description for the IPAM pool.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// ID of the IPAM pool.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// ID of the scope the pool belongs to.
    #[builder(into)]
    #[serde(rename = "ipamScopeId")]
    pub r#ipam_scope_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "ipamScopeType")]
    pub r#ipam_scope_type: Box<String>,
    /// Locale is the Region where your pool is available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region.
    #[builder(into)]
    #[serde(rename = "locale")]
    pub r#locale: Box<String>,
    #[builder(into)]
    #[serde(rename = "poolDepth")]
    pub r#pool_depth: Box<i32>,
    /// Defines whether or not IPv6 pool space is publicly advertisable over the internet.
    #[builder(into)]
    #[serde(rename = "publiclyAdvertisable")]
    pub r#publicly_advertisable: Box<bool>,
    /// ID of the source IPAM pool.
    #[builder(into)]
    #[serde(rename = "sourceIpamPoolId")]
    pub r#source_ipam_pool_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// Map of tags to assigned to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
}
