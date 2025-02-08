#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GuestPoliciesRecipeUpdateStepFileExec {
    /// A list of possible return values that the program can return to indicate a success. Defaults to [0].
    #[builder(into, default)]
    #[serde(rename = "allowedExitCodes")]
    pub r#allowed_exit_codes: Box<Option<Vec<i32>>>,
    /// Arguments to be passed to the provided executable.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// The id of the relevant artifact in the recipe.
    #[builder(into, default)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: Box<Option<String>>,
    /// The absolute path of the file on the local filesystem.
    #[builder(into, default)]
    #[serde(rename = "localPath")]
    pub r#local_path: Box<Option<String>>,
}
