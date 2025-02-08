#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerPoolLoadShedding {
    /// Percent of traffic to shed 0 - 100. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`, `random`. Defaults to `""`.
    #[builder(into, default)]
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    /// Percent of session traffic to shed 0 - 100. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`. Defaults to `""`.
    #[builder(into, default)]
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}
