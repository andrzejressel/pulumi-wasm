#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterLinuxProfileSshKey {
    /// The Public SSH Key used to access the cluster.
    #[builder(into)]
    #[serde(rename = "keyData")]
    pub r#key_data: Box<String>,
}