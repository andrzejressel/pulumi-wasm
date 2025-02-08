#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRouteFilterRule {
    /// The access type of the rule
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: Box<String>,
    /// The collection for bgp community values.
    #[builder(into)]
    #[serde(rename = "communities")]
    pub r#communities: Box<Vec<String>>,
    /// The Name of this Route Filter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Route Filter Rule Type.
    #[builder(into)]
    #[serde(rename = "ruleType")]
    pub r#rule_type: Box<String>,
}
