#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatabaseInstanceSettingsIpConfigurationPscConfig {
    /// List of consumer projects that are allow-listed for PSC connections to this instance. This instance can be connected to with PSC from any network in these projects. Each consumer project in this list may be represented by a project number (numeric) or by a project id (alphanumeric).
    #[builder(into, default)]
    #[serde(rename = "allowedConsumerProjects")]
    pub r#allowed_consumer_projects: Box<Option<Vec<String>>>,
    /// A comma-separated list of networks or a comma-separated list of network-project pairs. Each project in this list is represented by a project number (numeric) or by a project ID (alphanumeric). This allows Private Service Connect connections to be created automatically for the specified networks.
    #[builder(into, default)]
    #[serde(rename = "pscAutoConnections")]
    pub r#psc_auto_connections: Box<Option<Vec<super::super::types::sql::DatabaseInstanceSettingsIpConfigurationPscConfigPscAutoConnection>>>,
    /// Whether PSC connectivity is enabled for this instance.
    #[builder(into, default)]
    #[serde(rename = "pscEnabled")]
    pub r#psc_enabled: Box<Option<bool>>,
}
