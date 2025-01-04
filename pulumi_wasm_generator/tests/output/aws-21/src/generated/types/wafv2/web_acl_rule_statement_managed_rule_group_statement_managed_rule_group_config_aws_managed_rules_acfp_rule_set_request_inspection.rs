#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspection {
    /// The names of the fields in the request payload that contain your customer's primary physical address. See `address_fields` for more details.
    #[builder(into, default)]
    #[serde(rename = "addressFields")]
    pub r#address_fields: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionAddressFields>>,
    /// The name of the field in the request payload that contains your customer's email. See `email_field` for more details.
    #[builder(into, default)]
    #[serde(rename = "emailField")]
    pub r#email_field: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionEmailField>>,
    /// Details about your login page password field. See `password_field` for more details.
    #[builder(into, default)]
    #[serde(rename = "passwordField")]
    pub r#password_field: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionPasswordField>>,
    /// The payload type for your login endpoint, either JSON or form encoded.
    #[builder(into)]
    #[serde(rename = "payloadType")]
    pub r#payload_type: Box<String>,
    /// The names of the fields in the request payload that contain your customer's primary phone number. See `phone_number_fields` for more details.
    #[builder(into, default)]
    #[serde(rename = "phoneNumberFields")]
    pub r#phone_number_fields: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionPhoneNumberFields>>,
    /// Details about your login page username field. See `username_field` for more details.
    #[builder(into, default)]
    #[serde(rename = "usernameField")]
    pub r#username_field: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetRequestInspectionUsernameField>>,
}
