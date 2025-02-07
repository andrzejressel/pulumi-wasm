#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TagField {
    /// Holds the value for a tag field with boolean type.
    #[builder(into, default)]
    #[serde(rename = "boolValue")]
    pub r#bool_value: Box<Option<bool>>,
    /// (Output)
    /// The display name of this field
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// Holds the value for a tag field with double type.
    #[builder(into, default)]
    #[serde(rename = "doubleValue")]
    pub r#double_value: Box<Option<f64>>,
    /// Holds the value for a tag field with enum type. This value must be one of the allowed values in the definition of this enum.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "enumValue")]
    pub r#enum_value: Box<Option<String>>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "fieldName")]
    pub r#field_name: Box<String>,
    /// (Output)
    /// The order of this field with respect to other fields in this tag. For example, a higher value can indicate
    /// a more important field. The value can be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<i32>>,
    /// Holds the value for a tag field with string type.
    #[builder(into, default)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<Option<String>>,
    /// Holds the value for a tag field with timestamp type.
    #[builder(into, default)]
    #[serde(rename = "timestampValue")]
    pub r#timestamp_value: Box<Option<String>>,
}
