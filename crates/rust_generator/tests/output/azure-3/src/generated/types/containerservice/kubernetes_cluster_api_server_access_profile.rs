#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterApiServerAccessProfile {
    /// Set of authorized IP ranges to allow access to API server, e.g. ["198.51.100.0/24"].
    #[builder(into, default)]
    #[serde(rename = "authorizedIpRanges")]
    pub r#authorized_ip_ranges: Box<Option<Vec<String>>>,
}
