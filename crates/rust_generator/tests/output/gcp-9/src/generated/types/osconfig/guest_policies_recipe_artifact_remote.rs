#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuestPoliciesRecipeArtifactRemote {
    /// Must be provided if allowInsecure is false. SHA256 checksum in hex format, to compare to the checksum of the artifact.
    /// If the checksum is not empty and it doesn't match the artifact then the recipe installation fails before running any
    /// of the steps.
    #[builder(into, default)]
    #[serde(rename = "checkSum")]
    pub r#check_sum: Box<Option<String>>,
    /// URI from which to fetch the object. It should contain both the protocol and path following the format {protocol}://{location}.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
