#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceQuotaUsageMetricMetricDimension {
    #[builder(into, default)]
    #[serde(rename = "class")]
    pub r#class: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "resource")]
    pub r#resource: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
