#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMeshCertificates {
    /// Controls the issuance of workload mTLS certificates. It is enabled by default. Workload Identity is required, see workload_config.
    #[builder(into)]
    #[serde(rename = "enableCertificates")]
    pub r#enable_certificates: Box<bool>,
}
