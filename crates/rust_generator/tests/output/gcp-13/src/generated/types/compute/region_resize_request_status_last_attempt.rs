#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionResizeRequestStatusLastAttempt {
    /// (Output)
    /// Fatal errors encountered during the queueing or provisioning phases of the ResizeRequest that caused the transition to the FAILED state. Contrary to the lastAttempt errors, this field is final and errors are never removed from here, as the ResizeRequest is not going to retry.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errors")]
    pub r#errors: Box<Option<Vec<super::super::types::compute::RegionResizeRequestStatusLastAttemptError>>>,
}
