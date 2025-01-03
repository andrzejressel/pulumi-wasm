#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContent {
    /// Configures which bundles to install and their corresponding install specs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bundles")]
    pub r#bundles: Box<Option<Vec<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentBundle>>>,
    /// Configures the installation of the Template Library.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "templateLibrary")]
    pub r#template_library: Box<Option<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary>>,
}
