#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterServerlessV2ScalingConfiguration {
    /// The maximum Neptune Capacity Units (NCUs) for this cluster. Must be lower or equal than **128**. See [AWS Documentation](https://docs.aws.amazon.com/neptune/latest/userguide/neptune-serverless-capacity-scaling.html) for more details.
    #[builder(into, default)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<Option<f64>>,
    /// The minimum Neptune Capacity Units (NCUs) for this cluster. Must be greater or equal than **1**. See [AWS Documentation](https://docs.aws.amazon.com/neptune/latest/userguide/neptune-serverless-capacity-scaling.html) for more details.
    #[builder(into, default)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<Option<f64>>,
}
