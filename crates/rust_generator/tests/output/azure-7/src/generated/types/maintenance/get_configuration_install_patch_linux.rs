#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationInstallPatchLinux {
    /// List of Classification category of patches to be patched.
    #[builder(into)]
    #[serde(rename = "classificationsToIncludes")]
    pub r#classifications_to_includes: Box<Vec<String>>,
    /// List of package names to be excluded from patching.
    #[builder(into)]
    #[serde(rename = "packageNamesMaskToExcludes")]
    pub r#package_names_mask_to_excludes: Box<Vec<String>>,
    /// List of package names to be included for patching.
    #[builder(into)]
    #[serde(rename = "packageNamesMaskToIncludes")]
    pub r#package_names_mask_to_includes: Box<Vec<String>>,
}
