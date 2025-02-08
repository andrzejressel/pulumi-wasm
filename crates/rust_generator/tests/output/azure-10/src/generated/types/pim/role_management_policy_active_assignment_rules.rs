#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoleManagementPolicyActiveAssignmentRules {
    /// Must an assignment have an expiry date. `false` allows permanent assignment.
    #[builder(into, default)]
    #[serde(rename = "expirationRequired")]
    pub r#expiration_required: Box<Option<bool>>,
    /// The maximum length of time an assignment can be valid, as an ISO8601 duration. Permitted values: `P15D`, `P30D`, `P90D`, `P180D`, or `P365D`.
    #[builder(into, default)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Box<Option<String>>,
    /// Is a justification required to create new assignments.
    #[builder(into, default)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: Box<Option<bool>>,
    /// Is multi-factor authentication required to create new assignments.
    #[builder(into, default)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: Box<Option<bool>>,
    /// Is ticket information required to create new assignments.
    /// 
    /// One of `expiration_required` or `expire_after` must be provided.
    #[builder(into, default)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: Box<Option<bool>>,
}
