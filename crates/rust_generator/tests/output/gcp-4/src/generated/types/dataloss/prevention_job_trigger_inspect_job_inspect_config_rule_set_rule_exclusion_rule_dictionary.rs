#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleDictionary {
    /// Newline-delimited file of words in Cloud Storage. Only a single file is accepted.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudStoragePath")]
    pub r#cloud_storage_path: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleDictionaryCloudStoragePath>>,
    /// List of words or phrases to search for.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "wordList")]
    pub r#word_list: Box<Option<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleDictionaryWordList>>,
}
