#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxVirtualMachineScaleSetAutomaticInstanceRepair {
    /// The repair action that will be used for repairing unhealthy virtual machines in the scale set. Possible values include `Replace`, `Restart`, `Reimage`.
    /// 
    /// > **Note:**  Once the `action` field has been set it will always return the last value it was assigned if it is removed from the configuration file.
    /// 
    /// > **Note:**  If you wish to update the repair `action` of an existing `automatic_instance_repair` policy, you must first `disable` the `automatic_instance_repair` policy before you can re-enable the `automatic_instance_repair` policy with the new repair `action` defined.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
    /// Should the automatic instance repair be enabled on this Virtual Machine Scale Set?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Amount of time for which automatic repairs will be delayed. The grace period starts right after the VM is found unhealthy. Possible values are between `10` and `90` minutes. The time duration should be specified in `ISO 8601` format (e.g. `PT10M` to `PT90M`).
    /// 
    /// > **Note:**  Once the `grace_period` field has been set it will always return the last value it was assigned if it is removed from the configuration file.
    #[builder(into, default)]
    #[serde(rename = "gracePeriod")]
    pub r#grace_period: Box<Option<String>>,
}
