#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterSnapshotCopy {
    /// The destination region that you want to copy snapshots to.
    #[builder(into)]
    #[serde(rename = "destinationRegion")]
    pub r#destination_region: Box<String>,
    /// The name of the snapshot copy grant to use when snapshots of an AWS KMS-encrypted cluster are copied to the destination region.
    #[builder(into, default)]
    #[serde(rename = "grantName")]
    pub r#grant_name: Box<Option<String>>,
    /// The number of days to retain automated snapshots in the destination region after they are copied from the source region. Defaults to `7`.
    #[builder(into, default)]
    #[serde(rename = "retentionPeriod")]
    pub r#retention_period: Box<Option<i32>>,
}
