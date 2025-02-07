#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuestPoliciesRecipeInstallStepArchiveExtraction {
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: Box<String>,
    /// Directory to extract archive to. Defaults to / on Linux or C:\ on Windows.
    #[builder(into, default)]
    #[serde(rename = "destination")]
    pub r#destination: Box<Option<String>>,
    /// The type of the archive to extract.
    /// Possible values are: `TAR`, `TAR_GZIP`, `TAR_BZIP`, `TAR_LZMA`, `TAR_XZ`, `ZIP`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
