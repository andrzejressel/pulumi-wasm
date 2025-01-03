#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotTypeSlotTypeValues {
    /// Value of the slot type entry.
    /// See `sample_value` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "sampleValues")]
    pub r#sample_values: Box<Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSampleValue>>>,
    /// A list of additional values related to the slot type entry.
    /// See `synonyms` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "synonyms")]
    pub r#synonyms: Box<Option<Vec<super::super::types::lex::V2ModelsSlotTypeSlotTypeValuesSynonym>>>,
}
