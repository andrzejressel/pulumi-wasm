#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventSourceMappingFilterCriteria {
    /// A set of up to 5 filter. If an event satisfies at least one, Lambda sends the event to the function or adds it to the next batch. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "filters")]
    pub r#filters: Box<Option<Vec<super::super::types::lambda::EventSourceMappingFilterCriteriaFilter>>>,
}
