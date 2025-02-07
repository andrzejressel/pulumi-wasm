#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteFilterRule {
    /// The access type of the rule. The only possible value is `Allow`.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: Box<String>,
    /// The collection for bgp community values to filter on. e.g. ['12076:5010','12076:5020'].
    #[builder(into)]
    #[serde(rename = "communities")]
    pub r#communities: Box<Vec<String>>,
    /// The name of the route filter rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The rule type of the rule. The only possible value is `Community`.
    #[builder(into)]
    #[serde(rename = "ruleType")]
    pub r#rule_type: Box<String>,
}
