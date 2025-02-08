#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayHttpListener {
    /// One or more `custom_error_configuration` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "customErrorConfigurations")]
    pub r#custom_error_configurations: Box<Vec<super::super::types::network::GetApplicationGatewayHttpListenerCustomErrorConfiguration>>,
    /// The ID of the Web Application Firewall Policy which is used as an HTTP Listener for this Path Rule.
    #[builder(into)]
    #[serde(rename = "firewallPolicyId")]
    pub r#firewall_policy_id: Box<String>,
    /// The ID of the associated Frontend Configuration.
    #[builder(into)]
    #[serde(rename = "frontendIpConfigurationId")]
    pub r#frontend_ip_configuration_id: Box<String>,
    /// The Name of the Frontend IP Configuration used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "frontendIpConfigurationName")]
    pub r#frontend_ip_configuration_name: Box<String>,
    /// The ID of the associated Frontend Port.
    #[builder(into)]
    #[serde(rename = "frontendPortId")]
    pub r#frontend_port_id: Box<String>,
    /// The Name of the Frontend Port used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "frontendPortName")]
    pub r#frontend_port_name: Box<String>,
    /// The Hostname which is used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// A list of Hostname(s) used for this HTTP Listener. It allows special wildcard characters.
    #[builder(into)]
    #[serde(rename = "hostNames")]
    pub r#host_names: Box<Vec<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Protocol used for this Probe.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Is Server Name Indication required?
    #[builder(into)]
    #[serde(rename = "requireSni")]
    pub r#require_sni: Box<bool>,
    /// The ID of the associated SSL Certificate.
    #[builder(into)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: Box<String>,
    /// The name of the associated SSL Certificate which is used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "sslCertificateName")]
    pub r#ssl_certificate_name: Box<String>,
    /// The ID of the associated SSL Profile.
    #[builder(into)]
    #[serde(rename = "sslProfileId")]
    pub r#ssl_profile_id: Box<String>,
    /// The name of the associated SSL Profile which is used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "sslProfileName")]
    pub r#ssl_profile_name: Box<String>,
}
