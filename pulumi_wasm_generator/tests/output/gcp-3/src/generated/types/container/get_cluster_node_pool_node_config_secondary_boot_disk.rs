#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNodeConfigSecondaryBootDisk {
    /// Disk image to create the secondary boot disk from
    #[builder(into)]
    #[serde(rename = "diskImage")]
    pub r#disk_image: Box<String>,
    /// Mode for how the secondary boot disk is used.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
