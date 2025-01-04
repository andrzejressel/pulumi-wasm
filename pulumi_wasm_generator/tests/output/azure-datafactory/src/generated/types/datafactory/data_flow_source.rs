#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataFlowSource {
    /// A `dataset` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dataset")]
    pub r#dataset: Box<Option<super::super::types::datafactory::DataFlowSourceDataset>>,
    /// The description for the Data Flow Source.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A `flowlet` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "flowlet")]
    pub r#flowlet: Box<Option<super::super::types::datafactory::DataFlowSourceFlowlet>>,
    /// A `linked_service` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "linkedService")]
    pub r#linked_service: Box<Option<super::super::types::datafactory::DataFlowSourceLinkedService>>,
    /// The name for the Data Flow Source.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `rejected_linked_service` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "rejectedLinkedService")]
    pub r#rejected_linked_service: Box<Option<super::super::types::datafactory::DataFlowSourceRejectedLinkedService>>,
    /// A `schema_linked_service` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "schemaLinkedService")]
    pub r#schema_linked_service: Box<Option<super::super::types::datafactory::DataFlowSourceSchemaLinkedService>>,
}
