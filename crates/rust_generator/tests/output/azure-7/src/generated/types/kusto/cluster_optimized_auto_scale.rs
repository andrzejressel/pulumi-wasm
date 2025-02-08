#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterOptimizedAutoScale {
    /// The maximum number of allowed instances. Must between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "maximumInstances")]
    pub r#maximum_instances: Box<i32>,
    /// The minimum number of allowed instances. Must between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "minimumInstances")]
    pub r#minimum_instances: Box<i32>,
}
