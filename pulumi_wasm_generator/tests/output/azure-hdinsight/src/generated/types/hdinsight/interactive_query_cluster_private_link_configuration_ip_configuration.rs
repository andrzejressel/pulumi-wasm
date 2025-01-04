#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InteractiveQueryClusterPrivateLinkConfigurationIpConfiguration {
    /// Specifies the name for this HDInsight Interactive Query Cluster. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Indicates whether this IP configuration is primary.
    #[builder(into, default)]
    #[serde(rename = "primary")]
    pub r#primary: Box<Option<bool>>,
    /// The private IP address of the IP configuration.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The private IP allocation method. The only possible value now is `Dynamic`.
    #[builder(into, default)]
    #[serde(rename = "privateIpAllocationMethod")]
    pub r#private_ip_allocation_method: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
