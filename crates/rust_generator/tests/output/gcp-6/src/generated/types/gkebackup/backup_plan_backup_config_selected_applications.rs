#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPlanBackupConfigSelectedApplications {
    /// A list of namespaced Kubernetes resources.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "namespacedNames")]
    pub r#namespaced_names: Box<Vec<super::super::types::gkebackup::BackupPlanBackupConfigSelectedApplicationsNamespacedName>>,
}
