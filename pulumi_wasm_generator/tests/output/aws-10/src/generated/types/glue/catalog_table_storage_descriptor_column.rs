#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CatalogTableStorageDescriptorColumn {
    /// Free-form text comment.
    #[builder(into, default)]
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    /// Name of the Column.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Key-value pairs defining properties associated with the column.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
    /// Datatype of data in the Column.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
