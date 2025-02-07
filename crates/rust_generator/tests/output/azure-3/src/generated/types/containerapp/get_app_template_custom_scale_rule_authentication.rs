#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppTemplateCustomScaleRuleAuthentication {
    /// The name of the secret that contains the value for this environment variable.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "triggerParameter")]
    pub r#trigger_parameter: Box<String>,
}
