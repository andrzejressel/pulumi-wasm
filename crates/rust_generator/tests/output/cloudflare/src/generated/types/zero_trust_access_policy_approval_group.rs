#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
