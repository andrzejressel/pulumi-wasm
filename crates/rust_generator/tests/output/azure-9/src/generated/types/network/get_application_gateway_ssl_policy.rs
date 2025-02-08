#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewaySslPolicy {
    /// A List of accepted cipher suites.
    #[builder(into)]
    #[serde(rename = "cipherSuites")]
    pub r#cipher_suites: Box<Vec<String>>,
    /// A list of SSL Protocols which are disabled on this Application Gateway.
    #[builder(into)]
    #[serde(rename = "disabledProtocols")]
    pub r#disabled_protocols: Box<Vec<String>>,
    /// The minimum TLS version.
    #[builder(into)]
    #[serde(rename = "minProtocolVersion")]
    pub r#min_protocol_version: Box<String>,
    /// The Name of the Policy.
    #[builder(into)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Box<String>,
    /// The Type of the Policy.
    #[builder(into)]
    #[serde(rename = "policyType")]
    pub r#policy_type: Box<String>,
}
