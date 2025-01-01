#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AttachedClusterProxyConfig {
    /// The Kubernetes Secret resource that contains the HTTP(S) proxy configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "kubernetesSecret")]
    pub r#kubernetes_secret: Box<Option<super::super::types::container::AttachedClusterProxyConfigKubernetesSecret>>,
}
