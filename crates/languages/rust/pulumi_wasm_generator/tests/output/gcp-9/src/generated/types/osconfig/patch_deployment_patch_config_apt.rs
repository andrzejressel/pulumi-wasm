#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PatchDeploymentPatchConfigApt {
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
    /// By changing the type to DIST, the patching is performed using apt-get dist-upgrade instead.
    /// Possible values are: `DIST`, `UPGRADE`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
