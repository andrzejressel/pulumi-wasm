#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMeshCertificates {
    /// Controls the issuance of workload mTLS certificates. It is enabled by default. Workload Identity is required, see workload_config.
    #[builder(into)]
    #[serde(rename = "enableCertificates")]
    pub r#enable_certificates: Box<bool>,
}
