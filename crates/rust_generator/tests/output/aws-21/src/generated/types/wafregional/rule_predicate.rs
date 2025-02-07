#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RulePredicate {
    #[builder(into)]
    #[serde(rename = "dataId")]
    pub r#data_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "negated")]
    pub r#negated: Box<bool>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
