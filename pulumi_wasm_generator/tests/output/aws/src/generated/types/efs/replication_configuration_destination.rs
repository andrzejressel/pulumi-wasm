#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationConfigurationDestination {
    /// The availability zone in which the replica should be created. If specified, the replica will be created with One Zone storage. If omitted, regional storage will be used.
    #[builder(into, default)]
    #[serde(rename = "availabilityZoneName")]
    pub r#availability_zone_name: Box<Option<String>>,
    /// The ID of the destination file system for the replication. If no ID is provided, then EFS creates a new file system with the default settings.
    #[builder(into, default)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Box<Option<String>>,
    /// The Key ID, ARN, alias, or alias ARN of the KMS key that should be used to encrypt the replica file system. If omitted, the default KMS key for EFS `/aws/elasticfilesystem` will be used.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// The region in which the replica should be created.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
