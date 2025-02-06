#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudGatewayApiMetadata {
    /// Detailed description of the APIs available on the Gateway instance.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Location of additional documentation for the APIs available on the Gateway instance.
    #[builder(into, default)]
    #[serde(rename = "documentationUrl")]
    pub r#documentation_url: Box<Option<String>>,
    /// Base URL that API consumers will use to access APIs on the Gateway instance.
    #[builder(into, default)]
    #[serde(rename = "serverUrl")]
    pub r#server_url: Box<Option<String>>,
    /// Specifies the title describing the context of the APIs available on the Gateway instance.
    #[builder(into, default)]
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
    /// Specifies the version of APIs available on this Gateway instance.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
