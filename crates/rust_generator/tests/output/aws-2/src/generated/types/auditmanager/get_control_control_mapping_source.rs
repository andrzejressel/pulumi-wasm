#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetControlControlMappingSource {
    #[builder(into)]
    #[serde(rename = "sourceDescription")]
    pub r#source_description: Box<String>,
    #[builder(into)]
    #[serde(rename = "sourceFrequency")]
    pub r#source_frequency: Box<String>,
    #[builder(into)]
    #[serde(rename = "sourceId")]
    pub r#source_id: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "sourceKeyword")]
    pub r#source_keyword: Box<Option<super::super::types::auditmanager::GetControlControlMappingSourceSourceKeyword>>,
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "sourceSetUpOption")]
    pub r#source_set_up_option: Box<String>,
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "troubleshootingText")]
    pub r#troubleshooting_text: Box<String>,
}
