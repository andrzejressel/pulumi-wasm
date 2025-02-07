#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotTypeExternalSourceSettingGrammarSlotTypeSetting {
    /// Source of the grammar used to create the slot type.
    /// See `source` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<super::super::types::lex::V2ModelsSlotTypeExternalSourceSettingGrammarSlotTypeSettingSource>>,
}
