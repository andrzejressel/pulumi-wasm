#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseAllowCustomRequestHandling {
    /// The `insert_header` blocks used to define HTTP headers added to the request. See `insert_header` below for details.
    #[builder(into)]
    #[serde(rename = "insertHeaders")]
    pub r#insert_headers: Box<Vec<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseAllowCustomRequestHandlingInsertHeader>>,
}
