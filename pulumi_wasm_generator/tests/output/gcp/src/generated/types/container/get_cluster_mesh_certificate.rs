#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterMeshCertificate {
    /// When enabled the GKE Workload Identity Certificates controller and node agent will be deployed in the cluster.
    #[builder(into)]
    #[serde(rename = "enableCertificates")]
    pub r#enable_certificates: Box<bool>,
}
