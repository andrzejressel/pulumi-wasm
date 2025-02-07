#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTaskExecutionOverridesContainerOverrideEnvironment {
    /// The name of the key-value pair. For environment variables, this is the name of the environment variable.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value of the key-value pair. For environment variables, this is the value of the environment variable.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
