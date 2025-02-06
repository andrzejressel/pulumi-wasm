#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceVolumeConfigurationManagedEbsVolume {
    /// Whether the volume should be encrypted. Default value is `true`.
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<bool>>,
    /// Linux filesystem type for the volume. For volumes created from a snapshot, same filesystem type must be specified that the volume was using when the snapshot was created. Valid values are `ext3`, `ext4`, `xfs`. Default value is `xfs`.
    #[builder(into, default)]
    #[serde(rename = "fileSystemType")]
    pub r#file_system_type: Box<Option<String>>,
    /// Number of I/O operations per second (IOPS).
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// Amazon Resource Name (ARN) identifier of the Amazon Web Services Key Management Service key to use for Amazon EBS encryption.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Amazon ECS infrastructure IAM role that is used to manage your Amazon Web Services infrastructure. Recommended using the Amazon ECS-managed `AmazonECSInfrastructureRolePolicyForVolumes` IAM policy with this role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Size of the volume in GiB. You must specify either a `size_in_gb` or a `snapshot_id`. You can optionally specify a volume size greater than or equal to the snapshot size.
    #[builder(into, default)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: Box<Option<i32>>,
    /// Snapshot that Amazon ECS uses to create the volume. You must specify either a `size_in_gb` or a `snapshot_id`.
    #[builder(into, default)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Box<Option<String>>,
    /// The tags to apply to the volume. See below.
    #[builder(into, default)]
    #[serde(rename = "tagSpecifications")]
    pub r#tag_specifications: Box<Option<Vec<super::super::types::ecs::ServiceVolumeConfigurationManagedEbsVolumeTagSpecification>>>,
    /// Throughput to provision for a volume, in MiB/s, with a maximum of 1,000 MiB/s.
    #[builder(into, default)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<Option<i32>>,
    /// Volume type.
    #[builder(into, default)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<Option<String>>,
}
