#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResizeRequestStatusLastAttempt {
    /// (Output)
    /// [Output only] Fatal errors encountered during the queueing or provisioning phases of the ResizeRequest that caused the transition to the FAILED state. Contrary to the lastAttempt errors, this field is final and errors are never removed from here, as the ResizeRequest is not going to retry.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errors")]
    pub r#errors: Box<Option<Vec<super::super::types::compute::ResizeRequestStatusLastAttemptError>>>,
}
