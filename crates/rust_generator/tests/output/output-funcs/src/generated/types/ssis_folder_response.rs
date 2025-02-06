#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SsisFolderResponse {
    /// Metadata description.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Metadata id.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Metadata name.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The type of SSIS object metadata.
    /// Expected value is 'Folder'.
    #[builder(skip)]
    #[serde(rename = "type")]
    r#type_: Box<super::constants::ConstStringFolder>,
}
