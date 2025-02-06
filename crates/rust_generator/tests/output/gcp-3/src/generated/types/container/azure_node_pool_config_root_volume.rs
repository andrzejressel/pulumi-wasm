#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AzureNodePoolConfigRootVolume {
    /// Optional. The size of the disk, in GiBs. When unspecified, a default value is provided. See the specific reference in the parent resource.
    #[builder(into, default)]
    #[serde(rename = "sizeGib")]
    pub r#size_gib: Box<Option<i32>>,
}
