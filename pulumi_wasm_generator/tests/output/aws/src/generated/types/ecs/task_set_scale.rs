#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskSetScale {
    /// The unit of measure for the scale value. Default: `PERCENT`.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
    /// The value, specified as a percent total of a service's `desiredCount`, to scale the task set. Defaults to `0` if not specified. Accepted values are numbers between 0.0 and 100.0.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<f64>>,
}
