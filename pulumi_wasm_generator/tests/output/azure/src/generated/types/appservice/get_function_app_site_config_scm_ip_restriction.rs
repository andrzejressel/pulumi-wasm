#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFunctionAppSiteConfigScmIpRestriction {
    /// Allow or Deny access for this IP range. Defaults to Allow.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<super::super::types::appservice::GetFunctionAppSiteConfigScmIpRestrictionHeaders>,
    /// The IP Address used for this IP Restriction in CIDR notation.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// The name of the Function App resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The priority for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// The Service Tag used for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "serviceTag")]
    pub r#service_tag: Box<String>,
    /// The Virtual Network Subnet ID used for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: Box<String>,
}