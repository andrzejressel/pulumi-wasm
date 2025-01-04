#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupPlanBackupConfigSelectedApplicationsNamespacedName {
    /// The name of a Kubernetes Resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The namespace of a Kubernetes Resource.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
}
