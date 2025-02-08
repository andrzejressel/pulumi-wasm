#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterClusterCertificate {
    /// The HSM hardware certificate issued (signed) by AWS CloudHSM.
    #[builder(into)]
    #[serde(rename = "awsHardwareCertificate")]
    pub r#aws_hardware_certificate: Box<String>,
    /// The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.
    #[builder(into)]
    #[serde(rename = "clusterCertificate")]
    pub r#cluster_certificate: Box<String>,
    /// The certificate signing request (CSR). Available only in UNINITIALIZED state.
    #[builder(into)]
    #[serde(rename = "clusterCsr")]
    pub r#cluster_csr: Box<String>,
    /// The HSM certificate issued (signed) by the HSM hardware.
    #[builder(into)]
    #[serde(rename = "hsmCertificate")]
    pub r#hsm_certificate: Box<String>,
    /// The HSM hardware certificate issued (signed) by the hardware manufacturer.
    /// The number of available cluster certificates may vary depending on state of the cluster.
    #[builder(into)]
    #[serde(rename = "manufacturerHardwareCertificate")]
    pub r#manufacturer_hardware_certificate: Box<String>,
}
