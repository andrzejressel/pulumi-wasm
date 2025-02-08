#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseChallenge {
    /// Defines custom handling for the web request. See `custom_request_handling` below for details.
    #[builder(into, default)]
    #[serde(rename = "customRequestHandling")]
    pub r#custom_request_handling: Box<Option<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseChallengeCustomRequestHandling>>,
}
