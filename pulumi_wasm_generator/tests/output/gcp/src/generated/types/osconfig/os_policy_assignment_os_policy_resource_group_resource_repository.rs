#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceRepository {
    /// An Apt Repository. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "apt")]
    pub r#apt: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryApt>>,
    /// A Goo Repository. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "goo")]
    pub r#goo: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryGoo>>,
    /// A Yum Repository. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "yum")]
    pub r#yum: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryYum>>,
    /// A Zypper Repository. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "zypper")]
    pub r#zypper: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepositoryZypper>>,
}
