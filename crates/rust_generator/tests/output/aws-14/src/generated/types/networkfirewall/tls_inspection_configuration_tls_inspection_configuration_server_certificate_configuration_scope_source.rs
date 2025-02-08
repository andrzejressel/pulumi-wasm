#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource {
    /// An IP address or a block of IP addresses in CIDR notation. AWS Network Firewall supports all address ranges for IPv4.
    #[builder(into)]
    #[serde(rename = "addressDefinition")]
    pub r#address_definition: Box<String>,
}
