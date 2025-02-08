#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetInstanceTypeInferenceAccelerator {
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    #[builder(into)]
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Box<String>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
