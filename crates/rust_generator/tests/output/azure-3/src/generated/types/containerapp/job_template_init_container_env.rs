#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateInitContainerEnv {
    /// The name of the environment variable.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Name of the Container App secret from which to pull the environment variable value.
    #[builder(into, default)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<Option<String>>,
    /// The value of the environment variable.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
