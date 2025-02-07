#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkgDeb {
    /// Whether dependencies should also be installed. -
    /// install when false: `dpkg -i package` - install when true: `apt-get update
    /// && apt-get -y install package.deb`
    #[builder(into, default)]
    #[serde(rename = "pullDeps")]
    pub r#pull_deps: Box<Option<bool>>,
    /// A deb package. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgDebSource>,
}
