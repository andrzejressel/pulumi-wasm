#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetOsProfileLinuxPatch {
    /// Specifies the assessment mode.
    #[builder(into)]
    #[serde(rename = "assessmentMode")]
    pub r#assessment_mode: Box<String>,
    /// Specifies the patch mode.
    #[builder(into)]
    #[serde(rename = "patchMode")]
    pub r#patch_mode: Box<String>,
}
