#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterMonitorMetrics {
    /// Specifies a comma-separated list of Kubernetes annotation keys that will be used in the resource's labels metric.
    #[builder(into, default)]
    #[serde(rename = "annotationsAllowed")]
    pub r#annotations_allowed: Box<Option<String>>,
    /// Specifies a Comma-separated list of additional Kubernetes label keys that will be used in the resource's labels metric.
    /// 
    /// > **Note:** Both properties `annotations_allowed` and `labels_allowed` are required if you are enabling Managed Prometheus with an existing Azure Monitor Workspace.
    #[builder(into, default)]
    #[serde(rename = "labelsAllowed")]
    pub r#labels_allowed: Box<Option<String>>,
}
