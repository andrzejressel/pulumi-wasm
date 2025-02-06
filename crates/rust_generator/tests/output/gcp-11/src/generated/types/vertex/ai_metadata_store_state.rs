#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiMetadataStoreState {
    /// (Output)
    /// The disk utilization of the MetadataStore in bytes.
    #[builder(into, default)]
    #[serde(rename = "diskUtilizationBytes")]
    pub r#disk_utilization_bytes: Box<Option<String>>,
}
