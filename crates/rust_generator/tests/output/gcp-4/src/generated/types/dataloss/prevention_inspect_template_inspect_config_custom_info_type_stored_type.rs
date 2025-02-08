#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionInspectTemplateInspectConfigCustomInfoTypeStoredType {
    /// Resource name of the requested StoredInfoType, for example `organizations/433245324/storedInfoTypes/432452342`
    /// or `projects/project-id/storedInfoTypes/432452342`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
