#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GuestPoliciesRecipeInstallStepFileCopy {
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: Box<String>,
    /// The absolute path on the instance to put the file.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<String>,
    /// Whether to allow this step to overwrite existing files.If this is false and the file already exists the file
    /// is not overwritten and the step is considered a success. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "overwrite")]
    pub r#overwrite: Box<Option<bool>>,
    /// Consists of three octal digits which represent, in order, the permissions of the owner, group, and other users
    /// for the file (similarly to the numeric mode used in the linux chmod utility). Each digit represents a three bit
    /// number with the 4 bit corresponding to the read permissions, the 2 bit corresponds to the write bit, and the one
    /// bit corresponds to the execute permission. Default behavior is 755.
    /// Below are some examples of permissions and their associated values:
    /// read, write, and execute: 7 read and execute: 5 read and write: 6 read only: 4
    #[builder(into, default)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
}
