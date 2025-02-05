#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudServiceNetwork {
    /// Specifies the Name of the resource group containing network resources of Azure Spring Cloud Apps. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "appNetworkResourceGroup")]
    pub r#app_network_resource_group: Box<Option<String>>,
    /// Specifies the ID of the Subnet which should host the Spring Boot Applications deployed in this Spring Cloud Service. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "appSubnetId")]
    pub r#app_subnet_id: Box<String>,
    /// A list of (at least 3) CIDR ranges (at least /16) which are used to host the Spring Cloud infrastructure, which must not overlap with any existing CIDR ranges in the Subnet. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "cidrRanges")]
    pub r#cidr_ranges: Box<Vec<String>>,
    /// Specifies the egress traffic type of the Spring Cloud Service. Possible values are `loadBalancer` and `userDefinedRouting`. Defaults to `loadBalancer`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "outboundType")]
    pub r#outbound_type: Box<Option<String>>,
    /// Ingress read time out in seconds.
    #[builder(into, default)]
    #[serde(rename = "readTimeoutSeconds")]
    pub r#read_timeout_seconds: Box<Option<i32>>,
    /// Specifies the Name of the resource group containing network resources of Azure Spring Cloud Service Runtime. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "serviceRuntimeNetworkResourceGroup")]
    pub r#service_runtime_network_resource_group: Box<Option<String>>,
    /// Specifies the ID of the Subnet where the Service Runtime components of the Spring Cloud Service will exist. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "serviceRuntimeSubnetId")]
    pub r#service_runtime_subnet_id: Box<String>,
}
