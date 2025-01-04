#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RestorePlanRestoreConfigRestoreOrderGroupKindDependency {
    /// The requiring group kind requires that the satisfying
    /// group kind be restored first.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requiring")]
    pub r#requiring: Box<super::super::types::gkebackup::RestorePlanRestoreConfigRestoreOrderGroupKindDependencyRequiring>,
    /// The satisfying group kind must be restored first
    /// in order to satisfy the dependency.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "satisfying")]
    pub r#satisfying: Box<super::super::types::gkebackup::RestorePlanRestoreConfigRestoreOrderGroupKindDependencySatisfying>,
}
