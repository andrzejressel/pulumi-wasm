#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GuestPoliciesRecipeArtifact {
    /// Defaults to false. When false, recipes are subject to validations based on the artifact type:
    /// Remote: A checksum must be specified, and only protocols with transport-layer security are permitted.
    /// GCS: An object generation number must be specified.
    #[builder(into, default)]
    #[serde(rename = "allowInsecure")]
    pub r#allow_insecure: Box<Option<bool>>,
    /// A Google Cloud Storage artifact.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gcs")]
    pub r#gcs: Box<Option<super::super::types::osconfig::GuestPoliciesRecipeArtifactGcs>>,
    /// Id of the artifact, which the installation and update steps of this recipe can reference.
    /// Artifacts in a recipe cannot have the same id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// A generic remote artifact.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "remote")]
    pub r#remote: Box<Option<super::super::types::osconfig::GuestPoliciesRecipeArtifactRemote>>,
}
