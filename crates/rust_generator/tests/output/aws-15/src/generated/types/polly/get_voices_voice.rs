#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVoicesVoice {
    /// Additional codes for languages available for the specified voice in addition to its default language.
    #[builder(into)]
    #[serde(rename = "additionalLanguageCodes")]
    pub r#additional_language_codes: Box<Vec<String>>,
    /// Gender of the voice.
    #[builder(into)]
    #[serde(rename = "gender")]
    pub r#gender: Box<String>,
    /// Amazon Polly assigned voice ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Language identification tag for filtering the list of voices returned. If not specified, all available voices are returned.
    #[builder(into)]
    #[serde(rename = "languageCode")]
    pub r#language_code: Box<String>,
    /// Human readable name of the language in English.
    #[builder(into)]
    #[serde(rename = "languageName")]
    pub r#language_name: Box<String>,
    /// Name of the voice.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies which engines are supported by a given voice.
    #[builder(into)]
    #[serde(rename = "supportedEngines")]
    pub r#supported_engines: Box<Vec<String>>,
}
