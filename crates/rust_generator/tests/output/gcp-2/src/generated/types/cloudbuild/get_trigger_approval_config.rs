#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTriggerApprovalConfig {
    /// Whether or not approval is needed. If this is set on a build, it will become pending when run,
    /// and will need to be explicitly approved to start.
    #[builder(into)]
    #[serde(rename = "approvalRequired")]
    pub r#approval_required: Box<bool>,
}
