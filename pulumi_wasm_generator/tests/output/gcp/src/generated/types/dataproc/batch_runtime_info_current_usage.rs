#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BatchRuntimeInfoCurrentUsage {
    /// (Output)
    /// Accelerator type being used, if any.
    #[builder(into, default)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<Option<String>>,
    /// (Output)
    /// Milli (one-thousandth) accelerator..
    #[builder(into, default)]
    #[serde(rename = "milliAccelerator")]
    pub r#milli_accelerator: Box<Option<String>>,
    /// (Output)
    /// Milli (one-thousandth) Dataproc Compute Units (DCUs).
    #[builder(into, default)]
    #[serde(rename = "milliDcu")]
    pub r#milli_dcu: Box<Option<String>>,
    /// (Output)
    /// Milli (one-thousandth) Dataproc Compute Units (DCUs) charged at premium tier.
    #[builder(into, default)]
    #[serde(rename = "milliDcuPremium")]
    pub r#milli_dcu_premium: Box<Option<String>>,
    /// (Output)
    /// Shuffle Storage in gigabytes (GB).
    #[builder(into, default)]
    #[serde(rename = "shuffleStorageGb")]
    pub r#shuffle_storage_gb: Box<Option<String>>,
    /// (Output)
    /// Shuffle Storage in gigabytes (GB) charged at premium tier.
    #[builder(into, default)]
    #[serde(rename = "shuffleStorageGbPremium")]
    pub r#shuffle_storage_gb_premium: Box<Option<String>>,
    /// (Output)
    /// The timestamp of the usage snapshot.
    #[builder(into, default)]
    #[serde(rename = "snapshotTime")]
    pub r#snapshot_time: Box<Option<String>>,
}
