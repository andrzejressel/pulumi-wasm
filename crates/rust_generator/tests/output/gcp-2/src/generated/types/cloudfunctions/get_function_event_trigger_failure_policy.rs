#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFunctionEventTriggerFailurePolicy {
    /// Whether the function should be retried on failure.
    #[builder(into)]
    #[serde(rename = "retry")]
    pub r#retry: Box<bool>,
}
