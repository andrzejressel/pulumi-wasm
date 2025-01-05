#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceRootBlockDevice {
    /// Whether the volume should be destroyed on instance termination. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<Option<bool>>,
    /// Device name, e.g., `/dev/sdh` or `xvdh`.
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// Whether to enable volume encryption. Defaults to `false`. Must be configured to perform drift detection.
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<bool>>,
    /// Amount of provisioned [IOPS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-io-characteristics.html). Only valid for volume_type of `io1`, `io2` or `gp3`.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// Amazon Resource Name (ARN) of the KMS Key to use when encrypting the volume. Must be configured to perform drift detection.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Map of tags to assign to the device.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
    #[builder(into, default)]
    #[serde(rename = "tagsAll")]
    pub r#tags_all: Box<Option<std::collections::HashMap<String, String>>>,
    /// Throughput to provision for a volume in mebibytes per second (MiB/s). This is only valid for `volume_type` of `gp3`.
    #[builder(into, default)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<Option<i32>>,
    /// ID of the volume. For example, the ID can be accessed like this, `aws_instance.web.root_block_device.0.volume_id`.
    #[builder(into, default)]
    #[serde(rename = "volumeId")]
    pub r#volume_id: Box<Option<String>>,
    /// Size of the volume in gibibytes (GiB).
    #[builder(into, default)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<Option<i32>>,
    /// Type of volume. Valid values include `standard`, `gp2`, `gp3`, `io1`, `io2`, `sc1`, or `st1`. Defaults to the volume type that the AMI uses.
    /// 
    /// Modifying the `encrypted` or `kms_key_id` settings of the `root_block_device` requires resource replacement.
    #[builder(into, default)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<Option<String>>,
}
