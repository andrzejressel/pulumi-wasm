#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContent {
    /// map of bundle name to BundleInstallSpec. The bundle name maps to the `bundleName` key in the `policycontroller.gke.io/constraintData` annotation on a constraint.
    #[builder(into, default)]
    #[serde(rename = "bundles")]
    pub r#bundles: Box<Option<Vec<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContentBundle>>>,
    /// Configures the installation of the Template Library. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "templateLibrary")]
    pub r#template_library: Box<Option<super::super::types::gkehub::FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary>>,
}
