#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAutonomousDatabasesAutonomousDatabasePropertyConnectionStringProfile {
    /// The current consumer group being used by the connection. 
    ///  Possible values:
    ///  CONSUMER_GROUP_UNSPECIFIED
    /// HIGH
    /// MEDIUM
    /// LOW
    /// TP
    /// TPURGENT
    #[builder(into)]
    #[serde(rename = "consumerGroup")]
    pub r#consumer_group: Box<String>,
    /// The display name for the database connection.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    /// The host name format being currently used in connection string. 
    ///  Possible values:
    ///  HOST_FORMAT_UNSPECIFIED
    /// FQDN
    /// IP
    #[builder(into)]
    #[serde(rename = "hostFormat")]
    pub r#host_format: Box<String>,
    /// This field indicates if the connection string is regional and is only
    /// applicable for cross-region Data Guard.
    #[builder(into)]
    #[serde(rename = "isRegional")]
    pub r#is_regional: Box<bool>,
    /// The protocol being used by the connection. 
    ///  Possible values:
    ///  PROTOCOL_UNSPECIFIED
    /// TCP
    /// TCPS
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// The current session mode of the connection. 
    ///  Possible values:
    ///  SESSION_MODE_UNSPECIFIED
    /// DIRECT
    /// INDIRECT
    #[builder(into)]
    #[serde(rename = "sessionMode")]
    pub r#session_mode: Box<String>,
    /// The syntax of the connection string. 
    ///  Possible values:
    ///  SYNTAX_FORMAT_UNSPECIFIED
    /// LONG
    /// EZCONNECT
    /// EZCONNECTPLUS
    #[builder(into)]
    #[serde(rename = "syntaxFormat")]
    pub r#syntax_format: Box<String>,
    /// This field indicates the TLS authentication type of the connection. 
    ///  Possible values:
    ///  TLS_AUTHENTICATION_UNSPECIFIED
    /// SERVER
    /// MUTUAL
    #[builder(into)]
    #[serde(rename = "tlsAuthentication")]
    pub r#tls_authentication: Box<String>,
    /// The value of the connection string.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
