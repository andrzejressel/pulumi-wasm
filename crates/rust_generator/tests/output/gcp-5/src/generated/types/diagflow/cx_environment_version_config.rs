#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxEnvironmentVersionConfig {
    /// Format: projects/{{project}}/locations/{{location}}/agents/{{agent}}/flows/{{flow}}/versions/{{version}}.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
