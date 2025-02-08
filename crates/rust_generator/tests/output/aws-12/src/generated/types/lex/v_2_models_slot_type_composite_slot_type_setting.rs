#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsSlotTypeCompositeSlotTypeSetting {
    /// Sub slots in the composite slot.
    /// See `sub_slots` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "subSlots")]
    pub r#sub_slots: Box<Option<Vec<super::super::types::lex::V2ModelsSlotTypeCompositeSlotTypeSettingSubSlot>>>,
}
