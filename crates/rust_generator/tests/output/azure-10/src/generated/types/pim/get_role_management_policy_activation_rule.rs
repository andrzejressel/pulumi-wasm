#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRoleManagementPolicyActivationRule {
    /// An `approval_stage` block as defined below.
    #[builder(into)]
    #[serde(rename = "approvalStages")]
    pub r#approval_stages: Box<Vec<super::super::types::pim::GetRoleManagementPolicyActivationRuleApprovalStage>>,
    /// (String) The maximum length of time an activated role can be valid, in an ISO8601 Duration format.
    #[builder(into)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: Box<String>,
    /// (Boolean) Is approval required for activation.
    #[builder(into)]
    #[serde(rename = "requireApproval")]
    pub r#require_approval: Box<bool>,
    /// (Boolean) Is a justification required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: Box<bool>,
    /// (Boolean) Is multi-factor authentication required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: Box<bool>,
    /// (Boolean) Is ticket information required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: Box<bool>,
    /// (String) The Entra ID Conditional Access context that must be present for activation.
    #[builder(into)]
    #[serde(rename = "requiredConditionalAccessAuthenticationContext")]
    pub r#required_conditional_access_authentication_context: Box<String>,
}
