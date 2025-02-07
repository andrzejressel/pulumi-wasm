#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspection {
    /// Configures inspection of the response body. See `body_contains` for more details.
    #[builder(into, default)]
    #[serde(rename = "bodyContains")]
    pub r#body_contains: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspectionBodyContains>>,
    /// Configures inspection of the response header.See `header` for more details.
    #[builder(into, default)]
    #[serde(rename = "header")]
    pub r#header: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspectionHeader>>,
    /// Configures inspection of the response JSON. See `json` for more details.
    #[builder(into, default)]
    #[serde(rename = "json")]
    pub r#json: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspectionJson>>,
    /// Configures inspection of the response status code.See `status_code` for more details.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspectionStatusCode>>,
}
