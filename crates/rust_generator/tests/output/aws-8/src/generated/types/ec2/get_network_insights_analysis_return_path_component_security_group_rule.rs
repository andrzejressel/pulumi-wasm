#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRule {
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<String>,
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRulePortRange>>,
    #[builder(into)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    #[builder(into)]
    #[serde(rename = "securityGroupId")]
    pub r#security_group_id: Box<String>,
}
