#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPoolSchemaAttribute {
    /// - Data type of the attribute (e.g., string, number).
    #[builder(into)]
    #[serde(rename = "attributeDataType")]
    pub r#attribute_data_type: Box<String>,
    /// - Whether the attribute is for developer use only.
    #[builder(into)]
    #[serde(rename = "developerOnlyAttribute")]
    pub r#developer_only_attribute: Box<bool>,
    /// - Whether the attribute can be changed after user creation.
    #[builder(into)]
    #[serde(rename = "mutable")]
    pub r#mutable: Box<bool>,
    /// - Name of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "numberAttributeConstraints")]
    pub r#number_attribute_constraints: Box<Vec<super::super::types::cognito::GetUserPoolSchemaAttributeNumberAttributeConstraint>>,
    /// - Whether the attribute is required during user registration.
    /// * number_attribute_constraints - Constraints for numeric attributes.
    /// * string_attribute_constraints - Constraints for string attributes.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Box<bool>,
    #[builder(into)]
    #[serde(rename = "stringAttributeConstraints")]
    pub r#string_attribute_constraints: Box<Vec<super::super::types::cognito::GetUserPoolSchemaAttributeStringAttributeConstraint>>,
}
