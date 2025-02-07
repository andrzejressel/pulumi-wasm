#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceEbsBlockDevice {
    /// If the root block device will be deleted on termination.
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<bool>,
    /// Physical name of the device.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// If the EBS volume is encrypted.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<bool>,
    /// `0` If the volume is not a provisioned IOPS image, otherwise the supported IOPS count.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Box<i32>,
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<String>,
    /// ID of the snapshot.
    #[builder(into)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Box<String>,
    /// Map of tags assigned to the Instance.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
    /// Throughput of the volume, in MiB/s.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<i32>,
    #[builder(into)]
    #[serde(rename = "volumeId")]
    pub r#volume_id: Box<String>,
    /// Size of the volume, in GiB.
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<i32>,
    /// Type of the volume.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<String>,
}
