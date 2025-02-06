#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomTargetTypeCustomActions {
    /// The Skaffold custom action responsible for deploy operations.
    #[builder(into)]
    #[serde(rename = "deployAction")]
    pub r#deploy_action: Box<String>,
    /// List of Skaffold modules Cloud Deploy will include in the Skaffold Config as required before performing diagnose.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "includeSkaffoldModules")]
    pub r#include_skaffold_modules: Box<Option<Vec<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModule>>>,
    /// The Skaffold custom action responsible for render operations. If not provided then Cloud Deploy will perform the render operations via `skaffold render`.
    #[builder(into, default)]
    #[serde(rename = "renderAction")]
    pub r#render_action: Box<Option<String>>,
}
