#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetMetastoreServiceHiveMetastoreConfig {
    /// A mapping of Hive metastore version to the auxiliary version configuration.
    /// When specified, a secondary Hive metastore service is created along with the primary service.
    /// All auxiliary versions must be less than the service's primary version.
    /// The key is the auxiliary service name and it must match the regular expression a-z?.
    /// This means that the first character must be a lowercase letter, and all the following characters must be hyphens, lowercase letters, or digits, except the last character, which cannot be a hyphen.
    #[builder(into)]
    #[serde(rename = "auxiliaryVersions")]
    pub r#auxiliary_versions: Box<Vec<super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfigAuxiliaryVersion>>,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).
    /// The mappings override system defaults (some keys cannot be overridden)
    #[builder(into)]
    #[serde(rename = "configOverrides")]
    pub r#config_overrides: Box<std::collections::HashMap<String, String>>,
    /// The protocol to use for the metastore service endpoint. If unspecified, defaults to 'THRIFT'. Default value: "THRIFT" Possible values: ["THRIFT", "GRPC"]
    #[builder(into)]
    #[serde(rename = "endpointProtocol")]
    pub r#endpoint_protocol: Box<String>,
    /// Information used to configure the Hive metastore service as a service principal in a Kerberos realm.
    #[builder(into)]
    #[serde(rename = "kerberosConfigs")]
    pub r#kerberos_configs: Box<Vec<super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfigKerberosConfig>>,
    /// The Hive metastore schema version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
