#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceClientConnectionConfig {
    /// Configuration to enforce connectors only (ex: AuthProxy) connections to the database.
    #[builder(into, default)]
    #[serde(rename = "requireConnectors")]
    pub r#require_connectors: Box<Option<bool>>,
    /// SSL config option for this instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sslConfig")]
    pub r#ssl_config: Box<Option<super::super::types::alloydb::InstanceClientConnectionConfigSslConfig>>,
}
