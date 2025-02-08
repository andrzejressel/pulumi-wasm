#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPoolNetworkConfigurationEndpointConfigurationNetworkSecurityGroupRule {
    /// The action that should be taken for a specified IP address, subnet range or tag.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: Box<String>,
    /// The priority for this rule.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// The source address prefix or tag to match for the rule.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefix")]
    pub r#source_address_prefix: Box<String>,
    /// The source port ranges to match for the rule.
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Box<Vec<String>>,
}
