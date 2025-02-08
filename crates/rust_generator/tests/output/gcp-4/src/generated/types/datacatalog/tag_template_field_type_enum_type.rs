#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TagTemplateFieldTypeEnumType {
    /// The set of allowed values for this enum. The display names of the
    /// values must be case-insensitively unique within this set. Currently,
    /// enum values can only be added to the list of allowed values. Deletion
    /// and renaming of enum values are not supported.
    /// Can have up to 500 allowed values.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "allowedValues")]
    pub r#allowed_values: Box<Vec<super::super::types::datacatalog::TagTemplateFieldTypeEnumTypeAllowedValue>>,
}
