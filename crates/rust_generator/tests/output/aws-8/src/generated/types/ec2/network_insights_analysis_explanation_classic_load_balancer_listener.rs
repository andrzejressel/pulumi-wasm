#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NetworkInsightsAnalysisExplanationClassicLoadBalancerListener {
    #[builder(into, default)]
    #[serde(rename = "instancePort")]
    pub r#instance_port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "loadBalancerPort")]
    pub r#load_balancer_port: Box<Option<i32>>,
}
