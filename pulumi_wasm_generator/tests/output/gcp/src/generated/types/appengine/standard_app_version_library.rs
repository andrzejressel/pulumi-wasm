#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardAppVersionLibrary {
    /// Name of the library. Example "django".
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Version of the library to select, or "latest".
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
