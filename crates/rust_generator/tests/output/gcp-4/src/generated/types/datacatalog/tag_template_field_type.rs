#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TagTemplateFieldType {
    /// Represents an enum type.
    /// Exactly one of `primitive_type` or `enum_type` must be set
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "enumType")]
    pub r#enum_type: Box<Option<super::super::types::datacatalog::TagTemplateFieldTypeEnumType>>,
    /// Represents primitive types - string, bool etc.
    /// Exactly one of `primitive_type` or `enum_type` must be set
    /// Possible values are: `DOUBLE`, `STRING`, `BOOL`, `TIMESTAMP`.
    #[builder(into, default)]
    #[serde(rename = "primitiveType")]
    pub r#primitive_type: Box<Option<String>>,
}
