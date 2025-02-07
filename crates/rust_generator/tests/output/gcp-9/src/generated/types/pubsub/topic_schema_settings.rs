#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicSchemaSettings {
    /// The encoding of messages validated against schema.
    /// Default value is `ENCODING_UNSPECIFIED`.
    /// Possible values are: `ENCODING_UNSPECIFIED`, `JSON`, `BINARY`.
    #[builder(into, default)]
    #[serde(rename = "encoding")]
    pub r#encoding: Box<Option<String>>,
    /// The name of the schema that messages published should be
    /// validated against. Format is projects/{project}/schemas/{schema}.
    /// The value of this field will be _deleted-schema_
    /// if the schema has been deleted.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<String>,
}
