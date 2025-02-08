#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGroupFilter {
    /// Attribute path that is used to specify which attribute name to search. Currently, `DisplayName` is the only valid attribute path.
    #[builder(into)]
    #[serde(rename = "attributePath")]
    pub r#attribute_path: Box<String>,
    /// Value for an attribute.
    #[builder(into)]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<String>,
}
