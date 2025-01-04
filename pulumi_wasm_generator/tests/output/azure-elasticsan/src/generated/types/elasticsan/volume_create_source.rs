#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeCreateSource {
    /// Specifies the ID of the source to create the Elastic SAN Volume from. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sourceId")]
    pub r#source_id: Box<String>,
    /// Specifies the type of the source to create the Elastic SAN Volume from. Possible values are `Disk`, `DiskRestorePoint`, `DiskSnapshot` and `VolumeSnapshot`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Box<String>,
}
