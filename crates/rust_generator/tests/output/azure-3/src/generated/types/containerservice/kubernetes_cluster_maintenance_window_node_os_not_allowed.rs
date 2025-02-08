#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KubernetesClusterMaintenanceWindowNodeOsNotAllowed {
    /// The end of a time span, formatted as an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Box<String>,
    /// The start of a time span, formatted as an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
}
