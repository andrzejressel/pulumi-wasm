#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkgMsiSource {
    /// Defaults to false. When false, files are
    /// subject to validations based on the file type: Remote: A checksum must be
    /// specified. Cloud Storage: An object generation number must be specified.
    #[builder(into, default)]
    #[serde(rename = "allowInsecure")]
    pub r#allow_insecure: Box<Option<bool>>,
    /// A Cloud Storage object. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "gcs")]
    pub r#gcs: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgMsiSourceGcs>>,
    /// A local path within the VM to use.
    #[builder(into, default)]
    #[serde(rename = "localPath")]
    pub r#local_path: Box<Option<String>>,
    /// A generic remote file. Structure is
    /// documented below.
    #[builder(into, default)]
    #[serde(rename = "remote")]
    pub r#remote: Box<Option<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkgMsiSourceRemote>>,
}
