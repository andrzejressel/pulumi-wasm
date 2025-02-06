#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkInsightsAnalysisForwardPathComponentAclRule {
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<String>,
    #[builder(into)]
    #[serde(rename = "egress")]
    pub r#egress: Box<bool>,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentAclRulePortRange>>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    #[builder(into)]
    #[serde(rename = "ruleAction")]
    pub r#rule_action: Box<String>,
    #[builder(into)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: Box<i32>,
}
