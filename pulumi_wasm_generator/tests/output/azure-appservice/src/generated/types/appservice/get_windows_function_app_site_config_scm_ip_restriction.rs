#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsFunctionAppSiteConfigScmIpRestriction {
    /// The action to take.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The description of the ip restriction rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Vec<super::super::types::appservice::GetWindowsFunctionAppSiteConfigScmIpRestrictionHeader>>,
    /// The CIDR notation of the IP or IP Range to match.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// The name of this Windows Function App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The priority value of this `ip_restriction`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// The Service Tag used for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "serviceTag")]
    pub r#service_tag: Box<String>,
    /// The subnet id which the Windows Function App is vNet Integrated with.
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: Box<String>,
}
