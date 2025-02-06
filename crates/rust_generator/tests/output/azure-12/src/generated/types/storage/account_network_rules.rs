#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountNetworkRules {
    /// Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Valid options are any combination of `Logging`, `Metrics`, `AzureServices`, or `None`.
    #[builder(into, default)]
    #[serde(rename = "bypasses")]
    pub r#bypasses: Box<Option<Vec<String>>>,
    /// Specifies the default action of allow or deny when no other rules match. Valid options are `Deny` or `Allow`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<String>,
    /// List of public IP or IP ranges in CIDR Format. Only IPv4 addresses are allowed. /31 CIDRs, /32 CIDRs, and Private IP address ranges (as defined in [RFC 1918](https://tools.ietf.org/html/rfc1918#section-3)), are not allowed.
    #[builder(into, default)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<String>>>,
    /// One or more `private_link_access` block as defined below.
    /// 
    /// > **Note:** If specifying `network_rules`, one of either `ip_rules` or `virtual_network_subnet_ids` must be specified and `default_action` must be set to `Deny`.
    /// 
    /// > **Note:** Network Rules can be defined either directly on the `azure.storage.Account` resource, or using the `azure.storage.AccountNetworkRules` resource - but the two cannot be used together. If both are used against the same Storage Account, spurious changes will occur. When managing Network Rules using this resource, to change from a `default_action` of `Deny` to `Allow` requires defining, rather than removing, the block.
    /// 
    /// > **Note:** The prefix of `ip_rules` must be between 0 and 30 and only supports public IP addresses.
    /// 
    /// > **Note:** [More information on Validation is available here](https://docs.microsoft.com/en-gb/azure/storage/blobs/storage-custom-domain-name)
    #[builder(into, default)]
    #[serde(rename = "privateLinkAccesses")]
    pub r#private_link_accesses: Box<Option<Vec<super::super::types::storage::AccountNetworkRulesPrivateLinkAccess>>>,
    /// A list of resource ids for subnets.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkSubnetIds")]
    pub r#virtual_network_subnet_ids: Box<Option<Vec<String>>>,
}
