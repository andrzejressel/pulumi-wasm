#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LinkLinkConfiguration {
    /// Configuration for filtering which log groups are to send log events from the source account to the monitoring account. See `log_group_configuration` Block for details.
    #[builder(into, default)]
    #[serde(rename = "logGroupConfiguration")]
    pub r#log_group_configuration: Box<Option<super::super::types::oam::LinkLinkConfigurationLogGroupConfiguration>>,
    /// Configuration for filtering which metric namespaces are to be shared from the source account to the monitoring account. See `metric_configuration` Block for details.
    #[builder(into, default)]
    #[serde(rename = "metricConfiguration")]
    pub r#metric_configuration: Box<Option<super::super::types::oam::LinkLinkConfigurationMetricConfiguration>>,
}
