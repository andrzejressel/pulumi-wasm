#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGroupWarmPool {
    /// List of instance reuse policy objects.
    #[builder(into)]
    #[serde(rename = "instanceReusePolicies")]
    pub r#instance_reuse_policies: Box<Vec<super::super::types::autoscaling::GetGroupWarmPoolInstanceReusePolicy>>,
    #[builder(into)]
    #[serde(rename = "maxGroupPreparedCapacity")]
    pub r#max_group_prepared_capacity: Box<i32>,
    /// Minimum number of instances to maintain in the warm pool.
    #[builder(into)]
    #[serde(rename = "minSize")]
    pub r#min_size: Box<i32>,
    /// Instance state to transition to after the lifecycle actions are complete.
    #[builder(into)]
    #[serde(rename = "poolState")]
    pub r#pool_state: Box<String>,
}
