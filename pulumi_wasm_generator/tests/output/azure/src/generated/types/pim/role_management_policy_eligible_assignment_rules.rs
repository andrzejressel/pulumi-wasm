#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoleManagementPolicyEligibleAssignmentRules {
    /// Must an assignment have an expiry date. `false` allows permanent assignment.
    #[builder(into, default)]
    #[serde(rename = "expirationRequired")]
    pub r#expiration_required: Box<Option<bool>>,
    /// The maximum length of time an assignment can be valid, as an ISO8601 duration. Permitted values: `P15D`, `P30D`, `P90D`, `P180D`, or `P365D`.
    /// 
    /// One of `expiration_required` or `expire_after` must be provided.
    #[builder(into, default)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Box<Option<String>>,
}
