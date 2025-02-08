#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAuthoritySubordinateConfigPemIssuerChain {
    /// Expected to be in leaf-to-root order according to RFC 5246.
    #[builder(into)]
    #[serde(rename = "pemCertificates")]
    pub r#pem_certificates: Box<Vec<String>>,
}
