#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVirtualNetworkGatewayVpnClientConfigurationRootCertificate {
    /// Specifies the name of the Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The public certificate of the root certificate
    /// authority. The certificate must be provided in Base-64 encoded X.509 format
    /// (PEM).
    #[builder(into)]
    #[serde(rename = "publicCertData")]
    pub r#public_cert_data: Box<String>,
}
