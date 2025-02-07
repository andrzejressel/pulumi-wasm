#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomActionTypeConfigurationProperty {
    /// The description of the action configuration property.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Whether the configuration property is a key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<bool>,
    /// The name of the action configuration property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Indicates that the property will be used in conjunction with PollForJobs.
    #[builder(into, default)]
    #[serde(rename = "queryable")]
    pub r#queryable: Box<Option<bool>>,
    /// Whether the configuration property is a required value.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Box<bool>,
    /// Whether the configuration property is secret.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<bool>,
    /// The type of the configuration property. Valid values: `String`, `Number`, `Boolean`
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
