#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AwsNodePoolConfigRootVolume {
    /// Optional. The number of I/O operations per second (IOPS) to provision for GP3 volume.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// Optional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to encrypt AWS EBS volumes. If not specified, the default Amazon managed key associated to the AWS region where this cluster runs will be used.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
    /// Optional. The size of the volume, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource.
    #[builder(into, default)]
    #[serde(rename = "sizeGib")]
    pub r#size_gib: Box<Option<i32>>,
    /// Optional. The throughput to provision for the volume, in MiB/s. Only valid if the volume type is GP3. If volume type is gp3 and throughput is not specified, the throughput will defaults to 125.
    #[builder(into, default)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<Option<i32>>,
    /// Optional. Type of the EBS volume. When unspecified, it defaults to GP2 volume. Possible values: VOLUME_TYPE_UNSPECIFIED, GP2, GP3
    #[builder(into, default)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<Option<String>>,
}
