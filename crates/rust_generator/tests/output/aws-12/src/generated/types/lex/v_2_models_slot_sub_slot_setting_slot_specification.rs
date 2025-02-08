#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecification {
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: Box<String>,
    /// Unique identifier assigned to the slot type.
    #[builder(into)]
    #[serde(rename = "slotTypeId")]
    pub r#slot_type_id: Box<String>,
    /// Elicitation setting details for constituent sub slots of a composite slot.
    /// See the `value_elicitation_setting` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "valueElicitationSettings")]
    pub r#value_elicitation_settings: Box<Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSetting>>>,
}
