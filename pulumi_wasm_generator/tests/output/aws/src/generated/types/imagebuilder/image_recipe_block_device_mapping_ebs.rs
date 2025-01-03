#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageRecipeBlockDeviceMappingEbs {
    /// Whether to delete the volume on termination. Defaults to unset, which is the value inherited from the parent image.
    #[builder(into, default)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<Option<String>>,
    /// Whether to encrypt the volume. Defaults to unset, which is the value inherited from the parent image.
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<String>>,
    /// Number of Input/Output (I/O) operations per second to provision for an `io1` or `io2` volume.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key for encryption.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Identifier of the EC2 Volume Snapshot.
    #[builder(into, default)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Box<Option<String>>,
    /// For GP3 volumes only. The throughput in MiB/s that the volume supports.
    #[builder(into, default)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<Option<i32>>,
    /// Size of the volume, in GiB.
    #[builder(into, default)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<Option<i32>>,
    /// Type of the volume. For example, `gp2` or `io2`.
    #[builder(into, default)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<Option<String>>,
}
