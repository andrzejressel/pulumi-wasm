#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PatchDeploymentPatchConfigWindowsUpdate {
    /// Only apply updates of these windows update classifications. If empty, all updates are applied.
    /// Each value may be one of: `CRITICAL`, `SECURITY`, `DEFINITION`, `DRIVER`, `FEATURE_PACK`, `SERVICE_PACK`, `TOOL`, `UPDATE_ROLLUP`, `UPDATE`.
    #[builder(into, default)]
    #[serde(rename = "classifications")]
    pub r#classifications: Box<Option<Vec<String>>>,
    /// List of KBs to exclude from update.
    #[builder(into, default)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<Vec<String>>>,
    /// An exclusive list of kbs to be updated. These are the only patches that will be updated.
    /// This field must not be used with other patch configurations.
    #[builder(into, default)]
    #[serde(rename = "exclusivePatches")]
    pub r#exclusive_patches: Box<Option<Vec<String>>>,
}
