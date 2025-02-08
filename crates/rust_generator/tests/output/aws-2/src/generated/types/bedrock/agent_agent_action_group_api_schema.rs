#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AgentAgentActionGroupApiSchema {
    /// JSON or YAML-formatted payload defining the OpenAPI schema for the action group.
    /// Only one of `payload` or `s3` can be specified.
    #[builder(into, default)]
    #[serde(rename = "payload")]
    pub r#payload: Box<Option<String>>,
    /// Details about the S3 object containing the OpenAPI schema for the action group. See `s3` Block for details.
    /// Only one of `s3` or `payload` can be specified.
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::bedrock::AgentAgentActionGroupApiSchemaS3>>,
}
