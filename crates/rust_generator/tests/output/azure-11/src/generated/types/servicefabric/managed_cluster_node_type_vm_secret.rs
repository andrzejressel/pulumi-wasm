#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedClusterNodeTypeVmSecret {
    /// One or more `certificates` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "certificates")]
    pub r#certificates: Box<Vec<super::super::types::servicefabric::ManagedClusterNodeTypeVmSecretCertificate>>,
    /// The ID of the Vault that contain the certificates.
    #[builder(into)]
    #[serde(rename = "vaultId")]
    pub r#vault_id: Box<String>,
}
