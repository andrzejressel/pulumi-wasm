#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetStandardSiteConfigIpRestriction {
    /// Does this restriction `Allow` or `Deny` access for this IP range.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// The `headers` block for this specific `ip_restriction` as defined below.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<super::super::types::logicapps::GetStandardSiteConfigIpRestrictionHeaders>,
    /// The IP Address used for this IP Restriction in CIDR notation.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The name of this Logic App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The priority for this IP Restriction. Restrictions are enforced in priority order.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// The Service Tag used for this IP Restriction.
    #[builder(into, default)]
    #[serde(rename = "serviceTag")]
    pub r#service_tag: Box<Option<String>>,
    /// The Virtual Network Subnet ID used for this IP Restriction.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: Box<Option<String>>,
}
