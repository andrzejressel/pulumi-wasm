#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterServicePrincipal {
    /// The Client ID for the Service Principal.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The Client Secret for the Service Principal.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<String>,
}
