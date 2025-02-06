#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentStorageConfig {
    /// Optional. Name of an existing Cloud Storage bucket to be used by the environment.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
}
