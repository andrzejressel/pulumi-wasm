#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetastoreServiceHiveMetastoreConfig {
    /// A mapping of Hive metastore version to the auxiliary version configuration.
    /// When specified, a secondary Hive metastore service is created along with the primary service.
    /// All auxiliary versions must be less than the service's primary version.
    /// The key is the auxiliary service name and it must match the regular expression a-z?.
    /// This means that the first character must be a lowercase letter, and all the following characters must be hyphens, lowercase letters, or digits, except the last character, which cannot be a hyphen.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "auxiliaryVersions")]
    pub r#auxiliary_versions: Box<Option<Vec<super::super::types::dataproc::MetastoreServiceHiveMetastoreConfigAuxiliaryVersion>>>,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).
    /// The mappings override system defaults (some keys cannot be overridden)
    #[builder(into, default)]
    #[serde(rename = "configOverrides")]
    pub r#config_overrides: Box<Option<std::collections::HashMap<String, String>>>,
    /// The protocol to use for the metastore service endpoint. If unspecified, defaults to `THRIFT`.
    /// Default value is `THRIFT`.
    /// Possible values are: `THRIFT`, `GRPC`.
    #[builder(into, default)]
    #[serde(rename = "endpointProtocol")]
    pub r#endpoint_protocol: Box<Option<String>>,
    /// Information used to configure the Hive metastore service as a service principal in a Kerberos realm.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "kerberosConfig")]
    pub r#kerberos_config: Box<Option<super::super::types::dataproc::MetastoreServiceHiveMetastoreConfigKerberosConfig>>,
    /// The Hive metastore schema version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
