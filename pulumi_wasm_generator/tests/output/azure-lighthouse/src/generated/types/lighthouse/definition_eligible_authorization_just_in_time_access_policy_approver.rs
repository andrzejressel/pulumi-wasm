#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DefinitionEligibleAuthorizationJustInTimeAccessPolicyApprover {
    /// The display name of the Azure Active Directory Principal for the approver.
    #[builder(into, default)]
    #[serde(rename = "principalDisplayName")]
    pub r#principal_display_name: Box<Option<String>>,
    /// The Principal ID of the Azure Active Directory principal for the approver.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
}