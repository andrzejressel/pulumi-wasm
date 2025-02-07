#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DiskAsyncReplicationSecondaryDisk {
    /// The secondary disk.
    #[builder(into)]
    #[serde(rename = "disk")]
    pub r#disk: Box<String>,
    /// Output-only. Status of replication on the secondary disk.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
