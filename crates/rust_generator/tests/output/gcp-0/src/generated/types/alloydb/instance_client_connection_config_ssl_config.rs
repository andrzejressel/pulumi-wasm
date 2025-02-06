#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceClientConnectionConfigSslConfig {
    /// SSL mode. Specifies client-server SSL/TLS connection behavior.
    /// Possible values are: `ENCRYPTED_ONLY`, `ALLOW_UNENCRYPTED_AND_ENCRYPTED`.
    #[builder(into, default)]
    #[serde(rename = "sslMode")]
    pub r#ssl_mode: Box<Option<String>>,
}
