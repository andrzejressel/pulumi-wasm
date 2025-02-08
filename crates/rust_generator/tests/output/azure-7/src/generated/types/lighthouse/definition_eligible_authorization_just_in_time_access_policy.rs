#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DefinitionEligibleAuthorizationJustInTimeAccessPolicy {
    /// An `approver` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "approvers")]
    pub r#approvers: Box<Option<Vec<super::super::types::lighthouse::DefinitionEligibleAuthorizationJustInTimeAccessPolicyApprover>>>,
    /// The maximum access duration in ISO 8601 format for just-in-time access requests. Defaults to `PT8H`.
    #[builder(into, default)]
    #[serde(rename = "maximumActivationDuration")]
    pub r#maximum_activation_duration: Box<Option<String>>,
    /// The multi-factor authorization provider to be used for just-in-time access requests. Possible value is `Azure`.
    /// 
    /// > **Note:** When this property isn't set, it would be set to `None`.
    #[builder(into, default)]
    #[serde(rename = "multiFactorAuthProvider")]
    pub r#multi_factor_auth_provider: Box<Option<String>>,
}
