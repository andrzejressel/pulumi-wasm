#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementRateBasedStatementScopeDownStatementLabelMatchStatement {
    /// String to match against.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Specify whether you want to match using the label name or just the namespace. Valid values are `LABEL` or `NAMESPACE`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
}
