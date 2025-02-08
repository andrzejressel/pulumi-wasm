#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentTarget {
    /// The root configuration file to use for this deployment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "config")]
    pub r#config: Box<super::super::types::deploymentmanager::DeploymentTargetConfig>,
    /// Specifies import files for this configuration. This can be
    /// used to import templates or other files. For example, you might
    /// import a text file in order to use the file in a template.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "imports")]
    pub r#imports: Box<Option<Vec<super::super::types::deploymentmanager::DeploymentTargetImport>>>,
}
