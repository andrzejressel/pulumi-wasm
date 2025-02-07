#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterWorkloadIdentityConfig {
    /// The workload pool to attach all Kubernetes service accounts to.
    /// 
    #[builder(into, default)]
    #[serde(rename = "workloadPool")]
    pub r#workload_pool: Box<Option<String>>,
}
