#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionEventTriggerFailurePolicy {
    /// Whether the function should be retried on failure. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "retry")]
    pub r#retry: Box<bool>,
}
