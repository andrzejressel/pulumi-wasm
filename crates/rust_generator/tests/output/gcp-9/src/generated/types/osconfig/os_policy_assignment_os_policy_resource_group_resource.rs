#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResource {
    /// Exec resource Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "exec")]
    pub r#exec: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExec>>,
    /// File resource Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "file")]
    pub r#file: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceFile>>,
    /// The id of the resource with the following restrictions:
    /// 
    /// *   Must contain only lowercase letters, numbers, and hyphens.
    /// *   Must start with a letter.
    /// *   Must be between 1-63 characters.
    /// *   Must end with a number or a letter.
    /// *   Must be unique within the OS policy.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Package resource Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "pkg")]
    pub r#pkg: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkg>>,
    /// Package repository resource Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "repository")]
    pub r#repository: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepository>>,
}
