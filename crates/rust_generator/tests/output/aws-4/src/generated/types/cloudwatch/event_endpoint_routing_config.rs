#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventEndpointRoutingConfig {
    /// Parameters used for failover. This includes what triggers failover and what happens when it's triggered. Documented below.
    #[builder(into)]
    #[serde(rename = "failoverConfig")]
    pub r#failover_config: Box<super::super::types::cloudwatch::EventEndpointRoutingConfigFailoverConfig>,
}
