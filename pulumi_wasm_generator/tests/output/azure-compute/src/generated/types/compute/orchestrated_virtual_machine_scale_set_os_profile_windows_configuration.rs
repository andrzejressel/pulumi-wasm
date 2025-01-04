#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileWindowsConfiguration {
    /// One or more `additional_unattend_content` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "additionalUnattendContents")]
    pub r#additional_unattend_contents: Box<Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationAdditionalUnattendContent>>>,
    /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: Box<String>,
    /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: Box<String>,
    /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the `name` field. If the value of the `name` field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "computerNamePrefix")]
    pub r#computer_name_prefix: Box<Option<String>>,
    /// Are automatic updates enabled for this Virtual Machine? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enableAutomaticUpdates")]
    pub r#enable_automatic_updates: Box<Option<bool>>,
    /// Should the VM be patched without requiring a reboot? Possible values are `true` or `false`. Defaults to `false`. For more information about hot patching please see the [product documentation](https://docs.microsoft.com/azure/automanage/automanage-hotpatch).
    /// 
    /// > **Note:** Hotpatching can only be enabled if the `patch_mode` is set to `AutomaticByPlatform`, the `provision_vm_agent` is set to `true`, your `source_image_reference` references a hotpatching enabled image, the VM's `sku_name` is set to a [Azure generation 2](https://docs.microsoft.com/azure/virtual-machines/generation-2#generation-2-vm-sizes) VM SKU and the `extension` contains an application health extension. An example of how to correctly configure a Virtual Machine Scale Set to provision a Windows Virtual Machine with hotpatching enabled can be found in the `./examples/orchestrated-vm-scale-set/hotpatching-enabled` directory within the GitHub Repository.
    #[builder(into, default)]
    #[serde(rename = "hotpatchingEnabled")]
    pub r#hotpatching_enabled: Box<Option<bool>>,
    /// Specifies the mode of VM Guest Patching for the virtual machines that are associated to the Virtual Machine Scale Set. Possible values are `AutomaticByPlatform` or `ImageDefault`. Defaults to `ImageDefault`.
    /// 
    /// > **Note:** If the `patch_assessment_mode` is set to `AutomaticByPlatform` then the `provision_vm_agent` field must be set to `true`.
    #[builder(into, default)]
    #[serde(rename = "patchAssessmentMode")]
    pub r#patch_assessment_mode: Box<Option<String>>,
    /// Specifies the mode of in-guest patching of this Windows Virtual Machine. Possible values are `Manual`, `AutomaticByOS` and `AutomaticByPlatform`. Defaults to `AutomaticByOS`. For more information on patch modes please see the [product documentation](https://docs.microsoft.com/azure/virtual-machines/automatic-vm-guest-patching#patch-orchestration-modes).
    /// 
    /// > **Note:** If `patch_mode` is set to `AutomaticByPlatform` the `provision_vm_agent` must be set to `true` and the `extension` must contain at least one application health extension.
    #[builder(into, default)]
    #[serde(rename = "patchMode")]
    pub r#patch_mode: Box<Option<String>>,
    /// Should the Azure VM Agent be provisioned on each Virtual Machine in the Scale Set? Defaults to `true`. Changing this value forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "provisionVmAgent")]
    pub r#provision_vm_agent: Box<Option<bool>>,
    /// One or more `secret` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationSecret>>>,
    /// Specifies the time zone of the virtual machine, the possible values are defined [here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
    #[builder(into, default)]
    #[serde(rename = "timezone")]
    pub r#timezone: Box<Option<String>>,
    /// One or more `winrm_listener` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "winrmListeners")]
    pub r#winrm_listeners: Box<Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationWinrmListener>>>,
}
