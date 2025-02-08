#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionInspectTemplateInspectConfigCustomInfoTypeDictionary {
    /// Newline-delimited file of words in Cloud Storage. Only a single file is accepted.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudStoragePath")]
    pub r#cloud_storage_path: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeDictionaryCloudStoragePath>>,
    /// List of words or phrases to search for.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "wordList")]
    pub r#word_list: Box<Option<super::super::types::dataloss::PreventionInspectTemplateInspectConfigCustomInfoTypeDictionaryWordList>>,
}
