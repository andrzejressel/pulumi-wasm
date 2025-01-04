#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowletDataFlowTransformation {
    /// A `dataset` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dataset")]
    pub r#dataset: Box<Option<super::super::types::datafactory::FlowletDataFlowTransformationDataset>>,
    /// The description for the Data Flow transformation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A `flowlet` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "flowlet")]
    pub r#flowlet: Box<Option<super::super::types::datafactory::FlowletDataFlowTransformationFlowlet>>,
    /// A `linked_service` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "linkedService")]
    pub r#linked_service: Box<Option<super::super::types::datafactory::FlowletDataFlowTransformationLinkedService>>,
    /// The name for the Data Flow transformation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
