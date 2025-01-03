#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StackInstancesOperationPreferences {
    /// How the concurrency level behaves during the operation execution. Valid values are `STRICT_FAILURE_TOLERANCE` and `SOFT_FAILURE_TOLERANCE`.
    #[builder(into, default)]
    #[serde(rename = "concurrencyMode")]
    pub r#concurrency_mode: Box<Option<String>>,
    /// Number of accounts, per region, for which this operation can fail before CloudFormation stops the operation in that region.
    #[builder(into, default)]
    #[serde(rename = "failureToleranceCount")]
    pub r#failure_tolerance_count: Box<Option<i32>>,
    /// Percentage of accounts, per region, for which this stack operation can fail before CloudFormation stops the operation in that region.
    #[builder(into, default)]
    #[serde(rename = "failureTolerancePercentage")]
    pub r#failure_tolerance_percentage: Box<Option<i32>>,
    /// Maximum number of accounts in which to perform this operation at one time.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentCount")]
    pub r#max_concurrent_count: Box<Option<i32>>,
    /// Maximum percentage of accounts in which to perform this operation at one time.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentPercentage")]
    pub r#max_concurrent_percentage: Box<Option<i32>>,
    /// Concurrency type of deploying stack sets operations in regions, could be in parallel or one region at a time. Valid values are `SEQUENTIAL` and `PARALLEL`.
    #[builder(into, default)]
    #[serde(rename = "regionConcurrencyType")]
    pub r#region_concurrency_type: Box<Option<String>>,
    /// Order of the regions where you want to perform the stack operation.
    #[builder(into, default)]
    #[serde(rename = "regionOrders")]
    pub r#region_orders: Box<Option<Vec<String>>>,
}
