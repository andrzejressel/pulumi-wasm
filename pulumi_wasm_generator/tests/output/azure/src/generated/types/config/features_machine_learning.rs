#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeaturesMachineLearning {
    #[builder(into, default)]
    #[serde(rename = "purgeSoftDeletedWorkspaceOnDestroy")]
    pub r#purge_soft_deleted_workspace_on_destroy: Box<Option<bool>>,
}