#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerProtocolDetails {
    /// Indicates the transport method for the AS2 messages. Currently, only `HTTP` is supported.
    #[builder(into, default)]
    #[serde(rename = "as2Transports")]
    pub r#as_2_transports: Box<Option<Vec<String>>>,
    /// Indicates passive mode, for FTP and FTPS protocols. Enter a single IPv4 address, such as the public IP address of a firewall, router, or load balancer.
    #[builder(into, default)]
    #[serde(rename = "passiveIp")]
    pub r#passive_ip: Box<Option<String>>,
    /// Use to ignore the error that is generated when the client attempts to use `SETSTAT` on a file you are uploading to an S3 bucket. Valid values: `DEFAULT`, `ENABLE_NO_OP`.
    #[builder(into, default)]
    #[serde(rename = "setStatOption")]
    pub r#set_stat_option: Box<Option<String>>,
    /// A property used with Transfer Family servers that use the FTPS protocol. Provides a mechanism to resume or share a negotiated secret key between the control and data connection for an FTPS session. Valid values: `DISABLED`, `ENABLED`, `ENFORCED`.
    #[builder(into, default)]
    #[serde(rename = "tlsSessionResumptionMode")]
    pub r#tls_session_resumption_mode: Box<Option<String>>,
}
