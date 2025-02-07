#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpecContainerEnvValueFromSecretKeyRef {
    /// A Cloud Secret Manager secret version. Must be 'latest' for the latest
    /// version or an integer for a specific version.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
