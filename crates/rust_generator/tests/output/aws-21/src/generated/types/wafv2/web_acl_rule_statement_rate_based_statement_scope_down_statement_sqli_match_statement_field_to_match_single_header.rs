#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchSingleHeader {
    /// Name of the query header to inspect. This setting must be provided as lower case characters.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
