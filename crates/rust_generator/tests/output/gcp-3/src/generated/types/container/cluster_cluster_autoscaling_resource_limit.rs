#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterAutoscalingResourceLimit {
    /// Maximum amount of the resource in the cluster.
    #[builder(into, default)]
    #[serde(rename = "maximum")]
    pub r#maximum: Box<Option<i32>>,
    /// Minimum amount of the resource in the cluster.
    #[builder(into, default)]
    #[serde(rename = "minimum")]
    pub r#minimum: Box<Option<i32>>,
    /// The type of the resource. For example, `cpu` and
    /// `memory`.  See the [guide to using Node Auto-Provisioning](https://cloud.google.com/kubernetes-engine/docs/how-to/node-auto-provisioning)
    /// for a list of types.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
}
