#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences {
    /// Commitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: `COMMITMENT_PLAN_UNSPECIFIED`, `ON_DEMAND`, `COMMITMENT_1_YEAR_MONTHLY_PAYMENTS`, `COMMITMENT_3_YEAR_MONTHLY_PAYMENTS`, `COMMITMENT_1_YEAR_UPFRONT_PAYMENT`, `COMMITMENT_3_YEAR_UPFRONT_PAYMENT`,
    #[builder(into, default)]
    #[serde(rename = "commitmentPlan")]
    pub r#commitment_plan: Box<Option<String>>,
    /// CPU overcommit ratio. Acceptable values are between 1.0 and 8.0, with 0.1 increment.
    #[builder(into, default)]
    #[serde(rename = "cpuOvercommitRatio")]
    pub r#cpu_overcommit_ratio: Box<Option<f64>>,
    /// Memory overcommit ratio. Acceptable values are 1.0, 1.25, 1.5, 1.75 and 2.0.
    #[builder(into, default)]
    #[serde(rename = "memoryOvercommitRatio")]
    pub r#memory_overcommit_ratio: Box<Option<f64>>,
    /// The Deduplication and Compression ratio is based on the logical (Used Before) space required to store data before applying deduplication and compression, in relation to the physical (Used After) space required after applying deduplication and compression. Specifically, the ratio is the Used Before space divided by the Used After space. For example, if the Used Before space is 3 GB, but the physical Used After space is 1 GB, the deduplication and compression ratio is 3x. Acceptable values are between 1.0 and 4.0.
    #[builder(into, default)]
    #[serde(rename = "storageDeduplicationCompressionRatio")]
    pub r#storage_deduplication_compression_ratio: Box<Option<f64>>,
}
