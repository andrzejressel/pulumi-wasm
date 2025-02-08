#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CaPoolIssuancePolicyAllowedIssuanceModes {
    /// When true, allows callers to create Certificates by specifying a CertificateConfig.
    #[builder(into)]
    #[serde(rename = "allowConfigBasedIssuance")]
    pub r#allow_config_based_issuance: Box<bool>,
    /// When true, allows callers to create Certificates by specifying a CSR.
    #[builder(into)]
    #[serde(rename = "allowCsrBasedIssuance")]
    pub r#allow_csr_based_issuance: Box<bool>,
}
