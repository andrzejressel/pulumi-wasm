#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetastoreServiceScalingConfig {
    /// Represents the autoscaling configuration of a metastore service.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "autoscalingConfig")]
    pub r#autoscaling_config: Box<Option<super::super::types::dataproc::MetastoreServiceScalingConfigAutoscalingConfig>>,
    /// Metastore instance sizes.
    /// Possible values are: `EXTRA_SMALL`, `SMALL`, `MEDIUM`, `LARGE`, `EXTRA_LARGE`.
    #[builder(into, default)]
    #[serde(rename = "instanceSize")]
    pub r#instance_size: Box<Option<String>>,
    /// Scaling factor, in increments of 0.1 for values less than 1.0, and increments of 1.0 for values greater than 1.0.
    #[builder(into, default)]
    #[serde(rename = "scalingFactor")]
    pub r#scaling_factor: Box<Option<f64>>,
}
