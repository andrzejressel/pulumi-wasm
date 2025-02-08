#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TrustConfigTrustStoreIntermediateCa {
    /// PEM intermediate certificate used for building up paths for validation.
    /// Each certificate provided in PEM format may occupy up to 5kB.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "pemCertificate")]
    pub r#pem_certificate: Box<Option<String>>,
}
