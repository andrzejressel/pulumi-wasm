#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigHostGceInstanceBoostConfig {
    /// An accelerator card attached to the boost instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Box<Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceBoostConfigAccelerator>>>,
    /// Size of the boot disk in GB. The minimum boot disk size is `30` GB. Defaults to `50` GB.
    #[builder(into, default)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Box<Option<i32>>,
    /// Whether to enable nested virtualization on the Compute Engine VMs backing boosted Workstations.
    /// See https://cloud.google.com/workstations/docs/reference/rest/v1beta/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization
    #[builder(into, default)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: Box<Option<bool>>,
    /// The id to be used for the boost config.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The type of machine that boosted VM instances will use—for example, e2-standard-4. For more information about machine types that Cloud Workstations supports, see the list of available machine types https://cloud.google.com/workstations/docs/available-machine-types. Defaults to e2-standard-4.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// Number of instances to pool for faster workstation boosting.
    #[builder(into, default)]
    #[serde(rename = "poolSize")]
    pub r#pool_size: Box<Option<i32>>,
}
