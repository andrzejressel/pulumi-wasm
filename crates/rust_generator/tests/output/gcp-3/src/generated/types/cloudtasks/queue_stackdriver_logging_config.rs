#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QueueStackdriverLoggingConfig {
    /// Specifies the fraction of operations to write to Stackdriver Logging.
    /// This field may contain any value between 0.0 and 1.0, inclusive. 0.0 is the
    /// default and means that no operations are logged.
    #[builder(into)]
    #[serde(rename = "samplingRatio")]
    pub r#sampling_ratio: Box<f64>,
}
