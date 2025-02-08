#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OriginRequestPolicyHeadersConfig {
    #[builder(into, default)]
    #[serde(rename = "headerBehavior")]
    pub r#header_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<super::super::types::cloudfront::OriginRequestPolicyHeadersConfigHeaders>>,
}
