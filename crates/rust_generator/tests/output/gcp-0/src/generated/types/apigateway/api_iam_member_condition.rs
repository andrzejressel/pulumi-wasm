#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiIamMemberCondition {
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
}
