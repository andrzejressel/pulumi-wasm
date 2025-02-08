#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RunBookDraft {
    /// A `publish_content_link` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "contentLink")]
    pub r#content_link: Box<Option<super::super::types::automation::RunBookDraftContentLink>>,
    #[builder(into, default)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: Box<Option<String>>,
    /// Whether the draft in edit mode.
    #[builder(into, default)]
    #[serde(rename = "editModeEnabled")]
    pub r#edit_mode_enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "lastModifiedTime")]
    pub r#last_modified_time: Box<Option<String>>,
    /// Specifies the output types of the runbook.
    #[builder(into, default)]
    #[serde(rename = "outputTypes")]
    pub r#output_types: Box<Option<Vec<String>>>,
    /// A list of `parameters` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::automation::RunBookDraftParameter>>>,
}
