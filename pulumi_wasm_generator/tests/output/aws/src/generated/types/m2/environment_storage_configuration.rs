#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentStorageConfiguration {
    #[builder(into, default)]
    #[serde(rename = "efs")]
    pub r#efs: Box<Option<super::super::types::m2::EnvironmentStorageConfigurationEfs>>,
    #[builder(into, default)]
    #[serde(rename = "fsx")]
    pub r#fsx: Box<Option<super::super::types::m2::EnvironmentStorageConfigurationFsx>>,
}