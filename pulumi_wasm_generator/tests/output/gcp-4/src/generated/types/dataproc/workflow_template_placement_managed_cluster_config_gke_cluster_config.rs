#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplatePlacementManagedClusterConfigGkeClusterConfig {
    /// A target for the deployment.
    #[builder(into, default)]
    #[serde(rename = "namespacedGkeDeploymentTarget")]
    pub r#namespaced_gke_deployment_target: Box<Option<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigGkeClusterConfigNamespacedGkeDeploymentTarget>>,
}
