#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SecuritySolutionRecommendationsEnabled {
    /// Is Principal Authentication enabled for the ACR repository? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "acrAuthentication")]
    pub r#acr_authentication: Box<Option<bool>>,
    /// Is Agent send underutilized messages enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "agentSendUnutilizedMsg")]
    pub r#agent_send_unutilized_msg: Box<Option<bool>>,
    /// Is Security related system configuration issues identified? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "baseline")]
    pub r#baseline: Box<Option<bool>>,
    /// Is IoT Edge Hub memory optimized? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "edgeHubMemOptimize")]
    pub r#edge_hub_mem_optimize: Box<Option<bool>>,
    /// Is logging configured for IoT Edge module? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "edgeLoggingOption")]
    pub r#edge_logging_option: Box<Option<bool>>,
    /// Is inconsistent module settings enabled for SecurityGroup? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "inconsistentModuleSettings")]
    pub r#inconsistent_module_settings: Box<Option<bool>>,
    /// is Azure IoT Security agent installed? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "installAgent")]
    pub r#install_agent: Box<Option<bool>>,
    /// Is Default IP filter policy denied? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "ipFilterDenyAll")]
    pub r#ip_filter_deny_all: Box<Option<bool>>,
    /// Is IP filter rule source allowable IP range too large? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "ipFilterPermissiveRule")]
    pub r#ip_filter_permissive_rule: Box<Option<bool>>,
    /// Is any ports open on the device? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "openPorts")]
    pub r#open_ports: Box<Option<bool>>,
    /// Does firewall policy exist which allow necessary communication to/from the device? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "permissiveFirewallPolicy")]
    pub r#permissive_firewall_policy: Box<Option<bool>>,
    /// Is only necessary addresses or ports are permitted in? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "permissiveInputFirewallRules")]
    pub r#permissive_input_firewall_rules: Box<Option<bool>>,
    /// Is only necessary addresses or ports are permitted out? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "permissiveOutputFirewallRules")]
    pub r#permissive_output_firewall_rules: Box<Option<bool>>,
    /// Is high level permissions are needed for the module? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "privilegedDockerOptions")]
    pub r#privileged_docker_options: Box<Option<bool>>,
    /// Is any credentials shared among devices? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "sharedCredentials")]
    pub r#shared_credentials: Box<Option<bool>>,
    /// Does TLS cipher suite need to be updated? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "vulnerableTlsCipherSuite")]
    pub r#vulnerable_tls_cipher_suite: Box<Option<bool>>,
}
