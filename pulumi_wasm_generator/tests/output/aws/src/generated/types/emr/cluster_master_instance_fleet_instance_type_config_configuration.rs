#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMasterInstanceFleetInstanceTypeConfigConfiguration {
    /// Classification within a configuration.
    #[builder(into, default)]
    #[serde(rename = "classification")]
    pub r#classification: Box<Option<String>>,
    /// Map of properties specified within a configuration classification.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<std::collections::HashMap<String, String>>>,
}