#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationTimePartConfig {
    /// The part of the time to keep.
    /// Possible values are: `YEAR`, `MONTH`, `DAY_OF_MONTH`, `DAY_OF_WEEK`, `WEEK_OF_YEAR`, `HOUR_OF_DAY`.
    #[builder(into, default)]
    #[serde(rename = "partToExtract")]
    pub r#part_to_extract: Box<Option<String>>,
}
