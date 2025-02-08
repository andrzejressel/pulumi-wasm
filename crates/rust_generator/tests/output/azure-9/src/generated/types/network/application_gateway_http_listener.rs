#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayHttpListener {
    /// One or more `custom_error_configuration` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "customErrorConfigurations")]
    pub r#custom_error_configurations: Box<Option<Vec<super::super::types::network::ApplicationGatewayHttpListenerCustomErrorConfiguration>>>,
    /// The ID of the Web Application Firewall Policy which should be used for this HTTP Listener.
    #[builder(into, default)]
    #[serde(rename = "firewallPolicyId")]
    pub r#firewall_policy_id: Box<Option<String>>,
    /// The ID of the associated Frontend Configuration.
    #[builder(into, default)]
    #[serde(rename = "frontendIpConfigurationId")]
    pub r#frontend_ip_configuration_id: Box<Option<String>>,
    /// The Name of the Frontend IP Configuration used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "frontendIpConfigurationName")]
    pub r#frontend_ip_configuration_name: Box<String>,
    /// The ID of the associated Frontend Port.
    #[builder(into, default)]
    #[serde(rename = "frontendPortId")]
    pub r#frontend_port_id: Box<Option<String>>,
    /// The Name of the Frontend Port use for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "frontendPortName")]
    pub r#frontend_port_name: Box<String>,
    /// The Hostname which should be used for this HTTP Listener. Setting this value changes Listener Type to 'Multi site'.
    #[builder(into, default)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<Option<String>>,
    /// A list of Hostname(s) should be used for this HTTP Listener. It allows special wildcard characters.
    /// 
    /// > **NOTE** The `host_names` and `host_name` are mutually exclusive and cannot both be set.
    #[builder(into, default)]
    #[serde(rename = "hostNames")]
    pub r#host_names: Box<Option<Vec<String>>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Name of the HTTP Listener.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Protocol to use for this HTTP Listener. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Should Server Name Indication be Required? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "requireSni")]
    pub r#require_sni: Box<Option<bool>>,
    /// The ID of the associated SSL Certificate.
    #[builder(into, default)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: Box<Option<String>>,
    /// The name of the associated SSL Certificate which should be used for this HTTP Listener.
    #[builder(into, default)]
    #[serde(rename = "sslCertificateName")]
    pub r#ssl_certificate_name: Box<Option<String>>,
    /// The ID of the associated SSL Profile.
    #[builder(into, default)]
    #[serde(rename = "sslProfileId")]
    pub r#ssl_profile_id: Box<Option<String>>,
    /// The name of the associated SSL Profile which should be used for this HTTP Listener.
    #[builder(into, default)]
    #[serde(rename = "sslProfileName")]
    pub r#ssl_profile_name: Box<Option<String>>,
}
