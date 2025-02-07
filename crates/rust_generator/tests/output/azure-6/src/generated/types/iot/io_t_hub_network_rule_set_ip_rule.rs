#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IoTHubNetworkRuleSetIpRule {
    /// The desired action for requests captured by this rule. Possible values are `Allow`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// The IP address range in CIDR notation for the ip rule.
    #[builder(into)]
    #[serde(rename = "ipMask")]
    pub r#ip_mask: Box<String>,
    /// The name of the ip rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
