#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleAppVersionAutomaticScalingDiskUtilization {
    /// Target bytes read per second.
    #[builder(into, default)]
    #[serde(rename = "targetReadBytesPerSecond")]
    pub r#target_read_bytes_per_second: Box<Option<i32>>,
    /// Target ops read per seconds.
    #[builder(into, default)]
    #[serde(rename = "targetReadOpsPerSecond")]
    pub r#target_read_ops_per_second: Box<Option<i32>>,
    /// Target bytes written per second.
    #[builder(into, default)]
    #[serde(rename = "targetWriteBytesPerSecond")]
    pub r#target_write_bytes_per_second: Box<Option<i32>>,
    /// Target ops written per second.
    #[builder(into, default)]
    #[serde(rename = "targetWriteOpsPerSecond")]
    pub r#target_write_ops_per_second: Box<Option<i32>>,
}
