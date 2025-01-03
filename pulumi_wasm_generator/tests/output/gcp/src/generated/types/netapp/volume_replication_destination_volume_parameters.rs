#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeReplicationDestinationVolumeParameters {
    /// Description for the destination volume.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Share name for destination volume. If not specified, name of source volume's share name will be used.
    #[builder(into, default)]
    #[serde(rename = "shareName")]
    pub r#share_name: Box<Option<String>>,
    /// Name of an existing storage pool for the destination volume with format: `projects/{{project}}/locations/{{location}}/storagePools/{{poolId}}`
    #[builder(into)]
    #[serde(rename = "storagePool")]
    pub r#storage_pool: Box<String>,
    /// Name for the destination volume to be created. If not specified, the name of the source volume will be used.
    #[builder(into, default)]
    #[serde(rename = "volumeId")]
    pub r#volume_id: Box<Option<String>>,
}
