#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HadoopClusterRolesWorkerNodeAutoscaleCapacity {
    /// The maximum number of worker nodes to autoscale to based on the cluster's activity.
    #[builder(into)]
    #[serde(rename = "maxInstanceCount")]
    pub r#max_instance_count: Box<i32>,
    /// The minimum number of worker nodes to autoscale to based on the cluster's activity.
    #[builder(into)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Box<i32>,
}