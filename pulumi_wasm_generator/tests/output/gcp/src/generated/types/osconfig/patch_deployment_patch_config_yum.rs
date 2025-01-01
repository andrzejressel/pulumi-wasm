#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PatchDeploymentPatchConfigYum {
    /// List of packages to exclude from update. These packages will be excluded.
    #[builder(into, default)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// An exclusive list of packages to be updated. These are the only packages that will be updated.
    /// If these packages are not installed, they will be ignored. This field cannot be specified with
    /// any other patch configuration fields.
    #[builder(into, default)]
    #[serde(rename = "exclusivePackages")]
    pub r#exclusive_packages: Box<Option<Vec<String>>>,
    /// Will cause patch to run yum update-minimal instead.
    #[builder(into, default)]
    #[serde(rename = "minimal")]
    pub r#minimal: Box<Option<bool>>,
    /// Adds the --security flag to yum update. Not supported on all platforms.
    #[builder(into, default)]
    #[serde(rename = "security")]
    pub r#security: Box<Option<bool>>,
}
