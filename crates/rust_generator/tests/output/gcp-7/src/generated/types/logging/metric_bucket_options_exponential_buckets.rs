#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricBucketOptionsExponentialBuckets {
    /// Must be greater than 1.
    #[builder(into)]
    #[serde(rename = "growthFactor")]
    pub r#growth_factor: Box<f64>,
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "numFiniteBuckets")]
    pub r#num_finite_buckets: Box<i32>,
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "scale")]
    pub r#scale: Box<f64>,
}
