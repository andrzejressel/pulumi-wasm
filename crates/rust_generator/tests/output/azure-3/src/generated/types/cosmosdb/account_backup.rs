#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountBackup {
    /// The interval in minutes between two backups. Possible values are between 60 and 1440. Defaults to `240`.
    #[builder(into, default)]
    #[serde(rename = "intervalInMinutes")]
    pub r#interval_in_minutes: Box<Option<i32>>,
    /// The time in hours that each backup is retained. Possible values are between 8 and 720. Defaults to `8`.
    #[builder(into, default)]
    #[serde(rename = "retentionInHours")]
    pub r#retention_in_hours: Box<Option<i32>>,
    /// The storage redundancy is used to indicate the type of backup residency. Possible values are `Geo`, `Local` and `Zone`. Defaults to `Geo`.
    /// 
    /// > **Note:** You can only configure `interval_in_minutes`, `retention_in_hours` and `storage_redundancy` when the `type` field is set to `Periodic`.
    #[builder(into, default)]
    #[serde(rename = "storageRedundancy")]
    pub r#storage_redundancy: Box<Option<String>>,
    /// The continuous backup tier. Possible values are `Continuous7Days` and `Continuous30Days`.
    #[builder(into, default)]
    #[serde(rename = "tier")]
    pub r#tier: Box<Option<String>>,
    /// The type of the `backup`. Possible values are `Continuous` and `Periodic`.
    /// 
    /// > **Note:** Migration of `Periodic` to `Continuous` is one-way, changing `Continuous` to `Periodic` forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
