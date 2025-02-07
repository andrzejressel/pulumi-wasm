#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterPersistenceConfig {
    /// AOF configuration. This field will be ignored if mode is not AOF.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "aofConfig")]
    pub r#aof_config: Box<Option<super::super::types::redis::ClusterPersistenceConfigAofConfig>>,
    /// Optional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.
    /// - DISABLED: 	Persistence (both backup and restore) is disabled for the cluster.
    /// - RDB: RDB based Persistence is enabled.
    /// - AOF: AOF based Persistence is enabled.
    /// Possible values are: `PERSISTENCE_MODE_UNSPECIFIED`, `DISABLED`, `RDB`, `AOF`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// RDB configuration. This field will be ignored if mode is not RDB.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rdbConfig")]
    pub r#rdb_config: Box<Option<super::super::types::redis::ClusterPersistenceConfigRdbConfig>>,
}
