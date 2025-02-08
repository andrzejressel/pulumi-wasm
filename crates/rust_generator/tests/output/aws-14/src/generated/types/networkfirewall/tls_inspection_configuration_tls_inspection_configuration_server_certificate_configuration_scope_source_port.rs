#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort {
    /// The lower limit of the port range. This must be less than or equal to the `to_port`.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: Box<i32>,
    /// The upper limit of the port range. This must be greater than or equal to the `from_port`.
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: Box<i32>,
}
