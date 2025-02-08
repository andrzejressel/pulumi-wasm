#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionTypeField {
    /// Whether to set the isEncrypted flag of the connection field definition.
    #[builder(into, default)]
    #[serde(rename = "isEncrypted")]
    pub r#is_encrypted: Box<Option<bool>>,
    /// Whether to set the isOptional flag of the connection field definition.
    #[builder(into, default)]
    #[serde(rename = "isOptional")]
    pub r#is_optional: Box<Option<bool>>,
    /// The name which should be used for this connection field definition.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of the connection field definition.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
