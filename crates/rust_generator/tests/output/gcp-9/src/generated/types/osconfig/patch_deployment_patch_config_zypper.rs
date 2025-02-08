#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PatchDeploymentPatchConfigZypper {
    /// Install only patches with these categories. Common categories include security, recommended, and feature.
    #[builder(into, default)]
    #[serde(rename = "categories")]
    pub r#categories: Box<Option<Vec<String>>>,
    /// List of packages to exclude from update.
    #[builder(into, default)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// An exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command.
    /// This field must not be used with any other patch configuration fields.
    #[builder(into, default)]
    #[serde(rename = "exclusivePatches")]
    pub r#exclusive_patches: Box<Option<Vec<String>>>,
    /// Install only patches with these severities. Common severities include critical, important, moderate, and low.
    #[builder(into, default)]
    #[serde(rename = "severities")]
    pub r#severities: Box<Option<Vec<String>>>,
    /// Adds the --with-optional flag to zypper patch.
    #[builder(into, default)]
    #[serde(rename = "withOptional")]
    pub r#with_optional: Box<Option<bool>>,
    /// Adds the --with-update flag, to zypper patch.
    #[builder(into, default)]
    #[serde(rename = "withUpdate")]
    pub r#with_update: Box<Option<bool>>,
}
