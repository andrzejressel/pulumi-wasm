#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotSubSlotSetting {
    /// Expression text for defining the constituent sub slots in the composite slot using logical `AND` and `OR` operators.
    #[builder(into, default)]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Specifications for the constituent sub slots of a composite slot.
    /// See the `slot_specification` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "slotSpecifications")]
    pub r#slot_specifications: Box<Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecification>>>,
}
