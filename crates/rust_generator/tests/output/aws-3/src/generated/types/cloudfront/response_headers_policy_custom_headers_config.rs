#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponseHeadersPolicyCustomHeadersConfig {
    #[builder(into, default)]
    #[serde(rename = "items")]
    pub r#items: Box<Option<Vec<super::super::types::cloudfront::ResponseHeadersPolicyCustomHeadersConfigItem>>>,
}
