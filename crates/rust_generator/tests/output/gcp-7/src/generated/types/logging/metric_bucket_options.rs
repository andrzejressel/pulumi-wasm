#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricBucketOptions {
    /// Specifies a set of buckets with arbitrary widths.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "explicitBuckets")]
    pub r#explicit_buckets: Box<Option<super::super::types::logging::MetricBucketOptionsExplicitBuckets>>,
    /// Specifies an exponential sequence of buckets that have a width that is proportional to the value of
    /// the lower bound. Each bucket represents a constant relative uncertainty on a specific value in the bucket.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "exponentialBuckets")]
    pub r#exponential_buckets: Box<Option<super::super::types::logging::MetricBucketOptionsExponentialBuckets>>,
    /// Specifies a linear sequence of buckets that all have the same width (except overflow and underflow).
    /// Each bucket represents a constant absolute uncertainty on the specific value in the bucket.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "linearBuckets")]
    pub r#linear_buckets: Box<Option<super::super::types::logging::MetricBucketOptionsLinearBuckets>>,
}
