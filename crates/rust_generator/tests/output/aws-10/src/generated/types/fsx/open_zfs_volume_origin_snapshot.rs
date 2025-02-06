#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OpenZfsVolumeOriginSnapshot {
    /// Specifies the strategy used when copying data from the snapshot to the new volume. Valid values are `CLONE`, `FULL_COPY`, `INCREMENTAL_COPY`.
    #[builder(into)]
    #[serde(rename = "copyStrategy")]
    pub r#copy_strategy: Box<String>,
    /// The Amazon Resource Name (ARN) of the origin snapshot.
    #[builder(into)]
    #[serde(rename = "snapshotArn")]
    pub r#snapshot_arn: Box<String>,
}
