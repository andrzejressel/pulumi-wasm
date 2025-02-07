#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceQuotaUsageMetricMetricDimension {
    #[builder(into)]
    #[serde(rename = "class")]
    pub r#class: Box<String>,
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Box<String>,
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
