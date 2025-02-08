#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AppTemplateAzureQueueScaleRuleAuthentication {
    /// The name of the Container App Secret to use for this Scale Rule Authentication.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
    /// The Trigger Parameter name to use the supply the value retrieved from the `secret_name`.
    #[builder(into)]
    #[serde(rename = "triggerParameter")]
    pub r#trigger_parameter: Box<String>,
}
