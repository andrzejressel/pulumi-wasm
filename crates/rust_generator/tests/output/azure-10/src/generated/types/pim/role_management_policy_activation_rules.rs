#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoleManagementPolicyActivationRules {
    /// An `approval_stage` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "approvalStage")]
    pub r#approval_stage: Box<Option<super::super::types::pim::RoleManagementPolicyActivationRulesApprovalStage>>,
    /// The maximum length of time an activated role can be valid, in an ISO8601 Duration format (e.g. `PT8H`). Valid range is `PT30M` to `PT23H30M`, in 30 minute increments, or `PT1D`.
    #[builder(into, default)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: Box<Option<String>>,
    /// Is approval required for activation. If `true` an `approval_stage` block must be provided.
    #[builder(into, default)]
    #[serde(rename = "requireApproval")]
    pub r#require_approval: Box<Option<bool>>,
    /// Is a justification required during activation of the role.
    #[builder(into, default)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: Box<Option<bool>>,
    /// Is multi-factor authentication required to activate the role. Conflicts with `required_conditional_access_authentication_context`.
    #[builder(into, default)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: Box<Option<bool>>,
    /// Is ticket information requrired during activation of the role.
    #[builder(into, default)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: Box<Option<bool>>,
    /// The Entra ID Conditional Access context that must be present for activation. Conflicts with `require_multifactor_authentication`.
    #[builder(into, default)]
    #[serde(rename = "requiredConditionalAccessAuthenticationContext")]
    pub r#required_conditional_access_authentication_context: Box<Option<String>>,
}
