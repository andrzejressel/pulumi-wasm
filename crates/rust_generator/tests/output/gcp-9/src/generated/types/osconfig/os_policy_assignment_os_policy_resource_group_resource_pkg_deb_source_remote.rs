#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourcePkgDebSourceRemote {
    /// SHA256 checksum of the remote file.
    #[builder(into, default)]
    #[serde(rename = "sha256Checksum")]
    pub r#sha_256_checksum: Box<Option<String>>,
    /// URI from which to fetch the object. It should contain
    /// both the protocol and path following the format `{protocol}://{location}`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
