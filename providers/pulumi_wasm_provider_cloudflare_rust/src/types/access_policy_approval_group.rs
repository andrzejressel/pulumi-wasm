#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessPolicyApprovalGroup {
    /// Number of approvals needed.
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: Box<i32>,
    /// List of emails to request approval from.
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Option<Vec<String>>>,
    #[serde(rename = "emailListUuid")]
    pub r#email_list_uuid: Box<Option<String>>,
}
