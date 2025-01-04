#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCatalogTablePartitionKey {
    /// Free-form text comment.
    #[builder(into)]
    #[serde(rename = "comment")]
    pub r#comment: Box<String>,
    /// Name of the table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Datatype of data in the Column.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
