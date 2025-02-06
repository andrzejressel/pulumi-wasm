#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingDefaultValueSpecification {
    /// List of default values.
    /// Amazon Lex chooses the default value to use in the order that they are presented in the list.
    /// See the `default_value_list` argument reference below.
    #[builder(into, default)]
    #[serde(rename = "defaultValueLists")]
    pub r#default_value_lists: Box<Option<Vec<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingDefaultValueSpecificationDefaultValueList>>>,
}
