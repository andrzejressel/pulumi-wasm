#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpnServerConfigurationRadius {
    /// One or more `client_root_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "clientRootCertificates")]
    pub r#client_root_certificates: Box<Vec<super::super::types::network::GetVpnServerConfigurationRadiusClientRootCertificate>>,
    /// One or more `server_root_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "serverRootCertificates")]
    pub r#server_root_certificates: Box<Vec<super::super::types::network::GetVpnServerConfigurationRadiusServerRootCertificate>>,
    /// One or more `server` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "servers")]
    pub r#servers: Box<Vec<super::super::types::network::GetVpnServerConfigurationRadiusServer>>,
}
