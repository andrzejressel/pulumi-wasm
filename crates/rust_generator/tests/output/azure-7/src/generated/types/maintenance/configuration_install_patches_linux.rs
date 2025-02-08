#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConfigurationInstallPatchesLinux {
    /// List of Classification category of patches to be patched. Possible values are `Critical`, `Security` and `Other`.
    #[builder(into, default)]
    #[serde(rename = "classificationsToIncludes")]
    pub r#classifications_to_includes: Box<Option<Vec<String>>>,
    /// List of package names to be excluded from patching.
    #[builder(into, default)]
    #[serde(rename = "packageNamesMaskToExcludes")]
    pub r#package_names_mask_to_excludes: Box<Option<Vec<String>>>,
    /// List of package names to be included for patching.
    #[builder(into, default)]
    #[serde(rename = "packageNamesMaskToIncludes")]
    pub r#package_names_mask_to_includes: Box<Option<Vec<String>>>,
}
