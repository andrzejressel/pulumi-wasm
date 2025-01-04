#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowletDataFlowSourceFlowlet {
    /// Specifies the reference data flow parameters from dataset.
    #[builder(into, default)]
    #[serde(rename = "datasetParameters")]
    pub r#dataset_parameters: Box<Option<String>>,
    /// The name for the Data Factory Flowlet.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A map of parameters to associate with the Data Factory Flowlet.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
}
