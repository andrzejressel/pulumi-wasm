#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFunctionLoggingConfig {
    #[builder(into)]
    #[serde(rename = "applicationLogLevel")]
    pub r#application_log_level: Box<String>,
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: Box<String>,
    #[builder(into)]
    #[serde(rename = "logGroup")]
    pub r#log_group: Box<String>,
    #[builder(into)]
    #[serde(rename = "systemLogLevel")]
    pub r#system_log_level: Box<String>,
}
