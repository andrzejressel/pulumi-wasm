#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetResponseInspectionBodyContains {
    /// Strings in the body of the response that indicate a failed login attempt.
    #[builder(into)]
    #[serde(rename = "failureStrings")]
    pub r#failure_strings: Box<Vec<String>>,
    /// Strings in the body of the response that indicate a successful login attempt.
    #[builder(into)]
    #[serde(rename = "successStrings")]
    pub r#success_strings: Box<Vec<String>>,
}
