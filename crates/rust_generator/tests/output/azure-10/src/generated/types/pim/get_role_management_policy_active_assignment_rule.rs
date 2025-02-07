#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRoleManagementPolicyActiveAssignmentRule {
    /// (Boolean) Must an assignment have an expiry date.
    #[builder(into)]
    #[serde(rename = "expirationRequired")]
    pub r#expiration_required: Box<bool>,
    /// (String) The maximum length of time an assignment can be valid, as an ISO8601 duration.
    #[builder(into)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Box<String>,
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
}
