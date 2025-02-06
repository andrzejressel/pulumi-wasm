#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterCertificate {
    /// The HSM hardware certificate issued (signed) by AWS CloudHSM.
    #[builder(into, default)]
    #[serde(rename = "awsHardwareCertificate")]
    pub r#aws_hardware_certificate: Box<Option<String>>,
    /// The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.
    #[builder(into, default)]
    #[serde(rename = "clusterCertificate")]
    pub r#cluster_certificate: Box<Option<String>>,
    /// The certificate signing request (CSR). Available only in `UNINITIALIZED` state after an HSM instance is added to the cluster.
    #[builder(into, default)]
    #[serde(rename = "clusterCsr")]
    pub r#cluster_csr: Box<Option<String>>,
    /// The HSM certificate issued (signed) by the HSM hardware.
    #[builder(into, default)]
    #[serde(rename = "hsmCertificate")]
    pub r#hsm_certificate: Box<Option<String>>,
    /// The HSM hardware certificate issued (signed) by the hardware manufacturer.
    #[builder(into, default)]
    #[serde(rename = "manufacturerHardwareCertificate")]
    pub r#manufacturer_hardware_certificate: Box<Option<String>>,
}
