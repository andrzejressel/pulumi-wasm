#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScaleSetRollingUpgradePolicy {
    /// The maximum percent of total virtual machine instances that will be upgraded simultaneously by the rolling upgrade in one batch. As this is a maximum, unhealthy instances in previous or future batches can cause the percentage of instances in a batch to decrease to ensure higher reliability. Defaults to `20`.
    #[builder(into, default)]
    #[serde(rename = "maxBatchInstancePercent")]
    pub r#max_batch_instance_percent: Box<Option<i32>>,
    /// The maximum percentage of the total virtual machine instances in the scale set that can be simultaneously unhealthy, either as a result of being upgraded, or by being found in an unhealthy state by the virtual machine health checks before the rolling upgrade aborts. This constraint will be checked prior to starting any batch. Defaults to `20`.
    #[builder(into, default)]
    #[serde(rename = "maxUnhealthyInstancePercent")]
    pub r#max_unhealthy_instance_percent: Box<Option<i32>>,
    /// The maximum percentage of upgraded virtual machine instances that can be found to be in an unhealthy state. This check will happen after each batch is upgraded. If this percentage is ever exceeded, the rolling update aborts. Defaults to `20`.
    #[builder(into, default)]
    #[serde(rename = "maxUnhealthyUpgradedInstancePercent")]
    pub r#max_unhealthy_upgraded_instance_percent: Box<Option<i32>>,
    /// The wait time between completing the update for all virtual machines in one batch and starting the next batch. The time duration should be specified in ISO 8601 format for duration (<https://en.wikipedia.org/wiki/ISO_8601#Durations>). Defaults to `PT0S` seconds represented as `PT0S`.
    #[builder(into, default)]
    #[serde(rename = "pauseTimeBetweenBatches")]
    pub r#pause_time_between_batches: Box<Option<String>>,
}
