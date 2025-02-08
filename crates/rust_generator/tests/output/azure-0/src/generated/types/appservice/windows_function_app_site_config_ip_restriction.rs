#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSiteConfigIpRestriction {
    /// The action to take. Possible values are `Allow` or `Deny`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// The Description of this IP Restriction.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A `headers` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<super::super::types::appservice::WindowsFunctionAppSiteConfigIpRestrictionHeaders>>,
    /// The CIDR notation of the IP or IP Range to match. For example: `10.0.0.0/24` or `192.168.10.1/32`
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The name which should be used for this `ip_restriction`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The priority value of this `ip_restriction`. Defaults to `65000`.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// The Service Tag used for this IP Restriction.
    #[builder(into, default)]
    #[serde(rename = "serviceTag")]
    pub r#service_tag: Box<Option<String>>,
    /// The Virtual Network Subnet ID used for this IP Restriction.
    /// 
    /// > **NOTE:** One and only one of `ip_address`, `service_tag` or `virtual_network_subnet_id` must be specified.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: Box<Option<String>>,
}
