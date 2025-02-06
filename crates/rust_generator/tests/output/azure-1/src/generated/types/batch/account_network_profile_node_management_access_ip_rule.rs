#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountNetworkProfileNodeManagementAccessIpRule {
    /// Specifies the action of the ip rule. The only possible value is `Allow`. Defaults to `Allow`.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// The CIDR block from which requests will match the rule.
    #[builder(into)]
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<String>,
}
