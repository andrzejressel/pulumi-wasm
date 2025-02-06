#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCodeSigningConfigPolicy {
    /// Code signing configuration policy for deployment validation failure.
    #[builder(into)]
    #[serde(rename = "untrustedArtifactOnDeployment")]
    pub r#untrusted_artifact_on_deployment: Box<String>,
}
