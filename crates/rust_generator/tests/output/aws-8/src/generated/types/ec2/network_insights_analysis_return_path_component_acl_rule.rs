#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkInsightsAnalysisReturnPathComponentAclRule {
    #[builder(into, default)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "egress")]
    pub r#egress: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAclRulePortRange>>>,
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ruleAction")]
    pub r#rule_action: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: Box<Option<i32>>,
}
