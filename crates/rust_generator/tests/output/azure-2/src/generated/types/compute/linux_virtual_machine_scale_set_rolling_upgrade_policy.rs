#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxVirtualMachineScaleSetRollingUpgradePolicy {
    /// Should the Virtual Machine Scale Set ignore the Azure Zone boundaries when constructing upgrade batches? Possible values are `true` or `false`.
    #[builder(into, default)]
    #[serde(rename = "crossZoneUpgradesEnabled")]
    pub r#cross_zone_upgrades_enabled: Box<Option<bool>>,
    /// The maximum percent of total virtual machine instances that will be upgraded simultaneously by the rolling upgrade in one batch. As this is a maximum, unhealthy instances in previous or future batches can cause the percentage of instances in a batch to decrease to ensure higher reliability.
    #[builder(into)]
    #[serde(rename = "maxBatchInstancePercent")]
    pub r#max_batch_instance_percent: Box<i32>,
    /// The maximum percentage of the total virtual machine instances in the scale set that can be simultaneously unhealthy, either as a result of being upgraded, or by being found in an unhealthy state by the virtual machine health checks before the rolling upgrade aborts. This constraint will be checked prior to starting any batch.
    #[builder(into)]
    #[serde(rename = "maxUnhealthyInstancePercent")]
    pub r#max_unhealthy_instance_percent: Box<i32>,
    /// The maximum percentage of upgraded virtual machine instances that can be found to be in an unhealthy state. This check will happen after each batch is upgraded. If this percentage is ever exceeded, the rolling update aborts.
    #[builder(into)]
    #[serde(rename = "maxUnhealthyUpgradedInstancePercent")]
    pub r#max_unhealthy_upgraded_instance_percent: Box<i32>,
    /// Create new virtual machines to upgrade the scale set, rather than updating the existing virtual machines. Existing virtual machines will be deleted once the new virtual machines are created for each batch. Possible values are `true` or `false`.
    /// 
    /// > **Note:** `overprovision` must be set to `false` when `maximum_surge_instances_enabled` is specified.
    #[builder(into, default)]
    #[serde(rename = "maximumSurgeInstancesEnabled")]
    pub r#maximum_surge_instances_enabled: Box<Option<bool>>,
    /// The wait time between completing the update for all virtual machines in one batch and starting the next batch. The time duration should be specified in ISO 8601 format.
    #[builder(into)]
    #[serde(rename = "pauseTimeBetweenBatches")]
    pub r#pause_time_between_batches: Box<String>,
    /// Upgrade all unhealthy instances in a scale set before any healthy instances. Possible values are `true` or `false`.
    #[builder(into, default)]
    #[serde(rename = "prioritizeUnhealthyInstancesEnabled")]
    pub r#prioritize_unhealthy_instances_enabled: Box<Option<bool>>,
}
