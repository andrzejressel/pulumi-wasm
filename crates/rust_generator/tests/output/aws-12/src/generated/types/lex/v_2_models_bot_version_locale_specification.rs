#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsBotVersionLocaleSpecification {
    #[builder(into)]
    #[serde(rename = "sourceBotVersion")]
    pub r#source_bot_version: Box<String>,
}
