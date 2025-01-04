#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineStorageConfigurationLogSettings {
    #[builder(into)]
    #[serde(rename = "defaultFilePath")]
    pub r#default_file_path: Box<String>,
    #[builder(into)]
    #[serde(rename = "luns")]
    pub r#luns: Box<Vec<i32>>,
}
