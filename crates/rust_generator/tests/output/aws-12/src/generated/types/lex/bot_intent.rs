#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BotIntent {
    /// The name of the intent. Must be less than or equal to 100 characters in length.
    #[builder(into)]
    #[serde(rename = "intentName")]
    pub r#intent_name: Box<String>,
    /// The version of the intent. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "intentVersion")]
    pub r#intent_version: Box<String>,
}
