#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobQueryUserDefinedFunctionResource {
    /// An inline resource that contains code for a user-defined function (UDF).
    /// Providing a inline code resource is equivalent to providing a URI for a file containing the same code.
    #[builder(into, default)]
    #[serde(rename = "inlineCode")]
    pub r#inline_code: Box<Option<String>>,
    /// A code resource to load from a Google Cloud Storage URI (gs://bucket/path).
    #[builder(into, default)]
    #[serde(rename = "resourceUri")]
    pub r#resource_uri: Box<Option<String>>,
}
