#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTargetGroupHealthCheck {
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Box<i32>,
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<i32>,
    #[builder(into)]
    #[serde(rename = "matcher")]
    pub r#matcher: Box<String>,
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<i32>,
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Box<i32>,
}
