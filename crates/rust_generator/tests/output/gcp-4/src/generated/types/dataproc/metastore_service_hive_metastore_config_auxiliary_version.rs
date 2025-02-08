#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct MetastoreServiceHiveMetastoreConfigAuxiliaryVersion {
    /// A mapping of Hive metastore configuration key-value pairs to apply to the auxiliary Hive metastore (configured in hive-site.xml) in addition to the primary version's overrides.
    /// If keys are present in both the auxiliary version's overrides and the primary version's overrides, the value from the auxiliary version's overrides takes precedence.
    #[builder(into, default)]
    #[serde(rename = "configOverrides")]
    pub r#config_overrides: Box<Option<std::collections::HashMap<String, String>>>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The Hive metastore version of the auxiliary service. It must be less than the primary Hive metastore service's version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
