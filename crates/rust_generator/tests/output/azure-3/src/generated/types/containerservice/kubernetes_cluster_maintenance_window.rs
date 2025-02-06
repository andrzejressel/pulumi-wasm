#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterMaintenanceWindow {
    /// One or more `allowed` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "alloweds")]
    pub r#alloweds: Box<Option<Vec<super::super::types::containerservice::KubernetesClusterMaintenanceWindowAllowed>>>,
    /// One or more `not_allowed` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "notAlloweds")]
    pub r#not_alloweds: Box<Option<Vec<super::super::types::containerservice::KubernetesClusterMaintenanceWindowNotAllowed>>>,
}
