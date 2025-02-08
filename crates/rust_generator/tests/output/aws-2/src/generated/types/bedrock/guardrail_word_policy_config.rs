#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuardrailWordPolicyConfig {
    /// A config for the list of managed words. See Managed Word Lists Config for more information.
    #[builder(into, default)]
    #[serde(rename = "managedWordListsConfigs")]
    pub r#managed_word_lists_configs: Box<Option<Vec<super::super::types::bedrock::GuardrailWordPolicyConfigManagedWordListsConfig>>>,
    /// List of custom word configs. See Words Config for more information.
    #[builder(into, default)]
    #[serde(rename = "wordsConfigs")]
    pub r#words_configs: Box<Option<Vec<super::super::types::bedrock::GuardrailWordPolicyConfigWordsConfig>>>,
}
