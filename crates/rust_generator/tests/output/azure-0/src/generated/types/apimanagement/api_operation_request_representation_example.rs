#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApiOperationRequestRepresentationExample {
    /// A long description for this example.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A URL that points to the literal example.
    #[builder(into, default)]
    #[serde(rename = "externalValue")]
    pub r#external_value: Box<Option<String>>,
    /// The name of this example.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A short description for this example.
    #[builder(into, default)]
    #[serde(rename = "summary")]
    pub r#summary: Box<Option<String>>,
    /// The example of the representation.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
