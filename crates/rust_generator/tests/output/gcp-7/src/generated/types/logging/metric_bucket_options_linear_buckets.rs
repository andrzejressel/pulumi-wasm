#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricBucketOptionsLinearBuckets {
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "numFiniteBuckets")]
    pub r#num_finite_buckets: Box<i32>,
    /// Lower bound of the first bucket.
    #[builder(into)]
    #[serde(rename = "offset")]
    pub r#offset: Box<f64>,
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: Box<f64>,
}
