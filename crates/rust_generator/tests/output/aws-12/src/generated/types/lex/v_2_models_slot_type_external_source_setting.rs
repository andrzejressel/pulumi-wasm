#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsSlotTypeExternalSourceSetting {
    /// Settings required for a slot type based on a grammar that you provide.
    /// See `grammar_slot_type_setting` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "grammarSlotTypeSetting")]
    pub r#grammar_slot_type_setting: Box<Option<super::super::types::lex::V2ModelsSlotTypeExternalSourceSettingGrammarSlotTypeSetting>>,
}
