#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CodeSigningConfigPolicies {
    /// Code signing configuration policy for deployment validation failure. If you set the policy to Enforce, Lambda blocks the deployment request if code-signing validation checks fail. If you set the policy to Warn, Lambda allows the deployment and creates a CloudWatch log. Valid values: `Warn`, `Enforce`. Default value: `Warn`.
    #[builder(into)]
    #[serde(rename = "untrustedArtifactOnDeployment")]
    pub r#untrusted_artifact_on_deployment: Box<String>,
}