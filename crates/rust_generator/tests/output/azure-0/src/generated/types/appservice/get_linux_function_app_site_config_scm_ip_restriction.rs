#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetLinuxFunctionAppSiteConfigScmIpRestriction {
    /// The action taken.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The description of the ip restriction rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// A `headers` block as defined above.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Vec<super::super::types::appservice::GetLinuxFunctionAppSiteConfigScmIpRestrictionHeader>>,
    /// The CIDR notation of the IP or IP Range matched.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// The name which should be used for this Linux Function App.
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
    /// The Virtual Network Subnet ID used for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: Box<String>,
}
