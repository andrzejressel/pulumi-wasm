#[derive(serde::Serialize)]
pub struct AccessPolicyApprovalGroup {
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: Box<i32>,
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Box<Option<Vec<String>>>,
    #[serde(rename = "emailListUuid")]
    pub r#email_list_uuid: Box<Option<String>>,
}