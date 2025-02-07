#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterPersistenceConfigAofConfig {
    /// Optional. Available fsync modes.
    /// - NO - Do not explicitly call fsync(). Rely on OS defaults.
    /// - EVERYSEC - Call fsync() once per second in a background thread. A balance between performance and durability.
    /// - ALWAYS - Call fsync() for earch write command.
    /// Possible values are: `APPEND_FSYNC_UNSPECIFIED`, `NO`, `EVERYSEC`, `ALWAYS`.
    #[builder(into, default)]
    #[serde(rename = "appendFsync")]
    pub r#append_fsync: Box<Option<String>>,
}
