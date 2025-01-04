#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BatchRuntimeInfoApproximateUsage {
    /// (Output)
    /// Accelerator type being used, if any.
    #[builder(into, default)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<Option<String>>,
    /// (Output)
    /// Accelerator usage in (milliAccelerator x seconds)
    #[builder(into, default)]
    #[serde(rename = "milliAcceleratorSeconds")]
    pub r#milli_accelerator_seconds: Box<Option<String>>,
    /// (Output)
    /// DCU (Dataproc Compute Units) usage in (milliDCU x seconds)
    #[builder(into, default)]
    #[serde(rename = "milliDcuSeconds")]
    pub r#milli_dcu_seconds: Box<Option<String>>,
    /// (Output)
    /// Shuffle storage usage in (GB x seconds)
    #[builder(into, default)]
    #[serde(rename = "shuffleStorageGbSeconds")]
    pub r#shuffle_storage_gb_seconds: Box<Option<String>>,
}
