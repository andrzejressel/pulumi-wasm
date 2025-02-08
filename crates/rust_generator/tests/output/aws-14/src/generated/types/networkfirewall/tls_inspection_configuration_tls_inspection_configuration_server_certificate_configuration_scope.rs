#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope {
    /// Set of configuration blocks describing the destination ports to inspect for. If not specified, this matches with any destination port. See Destination Ports below for details.
    #[builder(into, default)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Box<Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestinationPort>>>,
    /// Set of configuration blocks describing the destination IP address and address ranges to inspect for, in CIDR notation. If not specified, this matches with any destination address. See Destination below for details.
    #[builder(into, default)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeDestination>>>,
    /// Set of protocols to inspect for, specified using the protocol's assigned internet protocol number (IANA). Network Firewall currently supports TCP only. Valid values: `6`
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Vec<i32>>,
    /// Set of configuration blocks describing the source ports to inspect for. If not specified, this matches with any source port. See Source Ports below for details.
    #[builder(into, default)]
    #[serde(rename = "sourcePorts")]
    pub r#source_ports: Box<Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSourcePort>>>,
    /// Set of configuration blocks describing the source IP address and address ranges to inspect for, in CIDR notation. If not specified, this matches with any source address. See Source below for details.
    #[builder(into, default)]
    #[serde(rename = "sources")]
    pub r#sources: Box<Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScopeSource>>>,
}
