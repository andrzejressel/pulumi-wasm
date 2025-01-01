#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionAppSiteConfigIpRestriction {
    /// Does this restriction `Allow` or `Deny` access for this IP range. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// The `headers` block for this specific `ip_restriction` as defined below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<super::super::types::appservice::FunctionAppSiteConfigIpRestrictionHeaders>>,
    /// The IP Address used for this IP Restriction in CIDR notation.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The name for this IP Restriction.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The priority for this IP Restriction. Restrictions are enforced in priority order. By default, the priority is set to 65000 if not specified.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// The Service Tag used for this IP Restriction.
    #[builder(into, default)]
    #[serde(rename = "serviceTag")]
    pub r#service_tag: Box<Option<String>>,
    /// The Virtual Network Subnet ID used for this IP Restriction.
    /// 
    /// > **NOTE:** One of either `ip_address`, `service_tag` or `virtual_network_subnet_id` must be specified
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: Box<Option<String>>,
}
