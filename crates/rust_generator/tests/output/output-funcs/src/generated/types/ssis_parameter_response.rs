#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SsisParameterResponse {
    /// Parameter type.
    #[builder(into, default)]
    #[serde(rename = "dataType")]
    pub r#data_type: Box<Option<String>>,
    /// Default value of parameter.
    #[builder(into, default)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Box<Option<String>>,
    /// Parameter description.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Design default value of parameter.
    #[builder(into, default)]
    #[serde(rename = "designDefaultValue")]
    pub r#design_default_value: Box<Option<String>>,
    /// Parameter id.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<f64>>,
    /// Parameter name.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Whether parameter is required.
    #[builder(into, default)]
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
    /// Whether parameter is sensitive.
    #[builder(into, default)]
    #[serde(rename = "sensitive")]
    pub r#sensitive: Box<Option<bool>>,
    /// Default sensitive value of parameter.
    #[builder(into, default)]
    #[serde(rename = "sensitiveDefaultValue")]
    pub r#sensitive_default_value: Box<Option<String>>,
    /// Parameter value set.
    #[builder(into, default)]
    #[serde(rename = "valueSet")]
    pub r#value_set: Box<Option<bool>>,
    /// Parameter value type.
    #[builder(into, default)]
    #[serde(rename = "valueType")]
    pub r#value_type: Box<Option<String>>,
    /// Parameter reference variable.
    #[builder(into, default)]
    #[serde(rename = "variable")]
    pub r#variable: Box<Option<String>>,
}
