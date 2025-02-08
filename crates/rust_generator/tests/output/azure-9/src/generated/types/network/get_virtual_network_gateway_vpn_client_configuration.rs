#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNetworkGatewayVpnClientConfiguration {
    /// The client id of the Azure VPN application.
    /// See [Create an Active Directory (AD) tenant for P2S OpenVPN protocol connections](https://docs.microsoft.com/en-gb/azure/vpn-gateway/openvpn-azure-ad-tenant-multi-app) for values
    /// This setting is incompatible with the use of
    /// `root_certificate` and `revoked_certificate`, `radius_server_address`, and `radius_server_secret`.
    #[builder(into)]
    #[serde(rename = "aadAudience")]
    pub r#aad_audience: Box<String>,
    /// The STS url for your tenant
    /// This setting is incompatible with the use of
    /// `root_certificate` and `revoked_certificate`, `radius_server_address`, and `radius_server_secret`.
    #[builder(into)]
    #[serde(rename = "aadIssuer")]
    pub r#aad_issuer: Box<String>,
    /// AzureAD Tenant URL
    /// This setting is incompatible with the use of
    /// `root_certificate` and `revoked_certificate`, `radius_server_address`, and `radius_server_secret`.
    #[builder(into)]
    #[serde(rename = "aadTenant")]
    pub r#aad_tenant: Box<String>,
    /// The address space out of which IP addresses for
    /// vpn clients will be taken. You can provide more than one address space, e.g.
    /// in CIDR notation.
    #[builder(into)]
    #[serde(rename = "addressSpaces")]
    pub r#address_spaces: Box<Vec<String>>,
    /// The address of the Radius server.
    /// This setting is incompatible with the use of
    /// `aad_tenant`, `aad_audience`, `aad_issuer`, `root_certificate` and `revoked_certificate`.
    #[builder(into)]
    #[serde(rename = "radiusServerAddress")]
    pub r#radius_server_address: Box<String>,
    /// The secret used by the Radius server.
    /// This setting is incompatible with the use of
    /// `aad_tenant`, `aad_audience`, `aad_issuer`, `root_certificate` and `revoked_certificate`.
    #[builder(into)]
    #[serde(rename = "radiusServerSecret")]
    pub r#radius_server_secret: Box<String>,
    /// One or more `revoked_certificate` blocks which
    /// are defined below.
    /// This setting is incompatible with the use of
    /// `aad_tenant`, `aad_audience`, `aad_issuer`, `radius_server_address`, and `radius_server_secret`.
    #[builder(into)]
    #[serde(rename = "revokedCertificates")]
    pub r#revoked_certificates: Box<Vec<super::super::types::network::GetVirtualNetworkGatewayVpnClientConfigurationRevokedCertificate>>,
    /// One or more `root_certificate` blocks which are
    /// defined below. These root certificates are used to sign the client certificate
    /// used by the VPN clients to connect to the gateway.
    /// This setting is incompatible with the use of
    /// `aad_tenant`, `aad_audience`, `aad_issuer`, `radius_server_address`, and `radius_server_secret`.
    #[builder(into)]
    #[serde(rename = "rootCertificates")]
    pub r#root_certificates: Box<Vec<super::super::types::network::GetVirtualNetworkGatewayVpnClientConfigurationRootCertificate>>,
    /// List of the protocols supported by the vpn client.
    /// The supported values are `SSTP`, `IkeV2` and `OpenVPN`.
    #[builder(into)]
    #[serde(rename = "vpnClientProtocols")]
    pub r#vpn_client_protocols: Box<Vec<String>>,
}
