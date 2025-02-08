#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetCertificateConfiguration {
    /// Indicates whether a TLS/SSL certificate is generated for a fleet. Valid values are `DISABLED` and `GENERATED`. Default value is `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "certificateType")]
    pub r#certificate_type: Box<Option<String>>,
}
