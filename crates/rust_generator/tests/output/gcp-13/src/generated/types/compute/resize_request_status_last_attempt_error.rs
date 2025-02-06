#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResizeRequestStatusLastAttemptError {
    /// (Output)
    /// [Output Only] The array of errors encountered while processing this operation.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errors")]
    pub r#errors: Box<Option<Vec<super::super::types::compute::ResizeRequestStatusLastAttemptErrorError>>>,
}
