#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BrokerConfiguration {
    /// The Configuration ID.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Revision of the Configuration.
    #[builder(into, default)]
    #[serde(rename = "revision")]
    pub r#revision: Box<Option<i32>>,
}
