#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeEnrichmentParametersHttpParameters {
    #[builder(into, default)]
    #[serde(rename = "headerParameters")]
    pub r#header_parameters: Box<Option<std::collections::HashMap<String, String>>>,
    #[builder(into, default)]
    #[serde(rename = "pathParameterValues")]
    pub r#path_parameter_values: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "queryStringParameters")]
    pub r#query_string_parameters: Box<Option<std::collections::HashMap<String, String>>>,
}
