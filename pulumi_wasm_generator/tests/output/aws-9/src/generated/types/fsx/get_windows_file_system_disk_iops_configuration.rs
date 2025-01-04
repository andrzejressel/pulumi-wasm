#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWindowsFileSystemDiskIopsConfiguration {
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Box<i32>,
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
