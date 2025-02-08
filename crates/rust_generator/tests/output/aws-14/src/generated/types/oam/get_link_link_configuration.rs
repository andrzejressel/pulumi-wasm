#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLinkLinkConfiguration {
    /// Configuration for filtering which log groups are to send log events from the source account to the monitoring account. See `log_group_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "logGroupConfigurations")]
    pub r#log_group_configurations: Box<Vec<super::super::types::oam::GetLinkLinkConfigurationLogGroupConfiguration>>,
    /// Configuration for filtering which metric namespaces are to be shared from the source account to the monitoring account. See `metric_configuration` Block for details.
    #[builder(into)]
    #[serde(rename = "metricConfigurations")]
    pub r#metric_configurations: Box<Vec<super::super::types::oam::GetLinkLinkConfigurationMetricConfiguration>>,
}
