#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessPolicyApprovalGroup {
    /// Number of approvals needed.
    #[builder(into)]
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: Box<i32>,
    /// List of emails to request approval from.
    #[builder(into, default)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "emailListUuid")]
    pub r#email_list_uuid: Box<Option<String>>,
}
