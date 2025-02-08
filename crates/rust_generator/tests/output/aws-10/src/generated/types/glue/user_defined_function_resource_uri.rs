#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UserDefinedFunctionResourceUri {
    /// The type of the resource. can be one of `JAR`, `FILE`, and `ARCHIVE`.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
    /// The URI for accessing the resource.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
