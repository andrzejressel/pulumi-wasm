#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiConfigGatewayConfigBackendConfig {
    /// Google Cloud IAM service account used to sign OIDC tokens for backends that have authentication configured
    /// (https://cloud.google.com/service-infrastructure/docs/service-management/reference/rest/v1/services.configs#backend).
    #[builder(into)]
    #[serde(rename = "googleServiceAccount")]
    pub r#google_service_account: Box<String>,
}
