#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstancePerformanceConfigIopsPerTb {
    /// The instance max IOPS will be calculated by multiplying
    /// the capacity of the instance (TB) by max_iops_per_tb,
    /// and rounding to the nearest 1000. The instance max IOPS
    /// will be changed dynamically based on the instance
    /// capacity.
    #[builder(into)]
    #[serde(rename = "maxIopsPerTb")]
    pub r#max_iops_per_tb: Box<i32>,
}
