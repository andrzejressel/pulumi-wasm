#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesFleetManagerHubProfile {
    #[builder(into)]
    #[serde(rename = "dnsPrefix")]
    pub r#dns_prefix: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "kubernetesVersion")]
    pub r#kubernetes_version: Box<Option<String>>,
}
