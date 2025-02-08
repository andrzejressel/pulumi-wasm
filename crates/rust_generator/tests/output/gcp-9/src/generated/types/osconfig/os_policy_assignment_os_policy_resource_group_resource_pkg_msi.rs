#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkgMsi {
    /// Additional properties to use during installation.
    /// This should be in the format of Property=Setting. Appended to the defaults
    /// of `ACTION=INSTALL REBOOT=ReallySuppress`.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<Vec<String>>>,
    /// The MSI package. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgMsiSource>,
}
