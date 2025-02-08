#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileLinuxConfiguration {
    /// The Password which should be used for the local-administrator on this Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: Box<Option<String>>,
    /// A `admin_ssh_key` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "adminSshKeys")]
    pub r#admin_ssh_keys: Box<Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileLinuxConfigurationAdminSshKey>>>,
    /// The username of the local administrator on each Virtual Machine Scale Set instance. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: Box<String>,
    /// The prefix which should be used for the name of the Virtual Machines in this Scale Set. If unspecified this defaults to the value for the name field. If the value of the name field is not a valid `computer_name_prefix`, then you must specify `computer_name_prefix`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "computerNamePrefix")]
    pub r#computer_name_prefix: Box<Option<String>>,
    /// When an `admin_password` is specified `disable_password_authentication` must be set to `false`. Defaults to `true`.
    /// 
    /// > **Note:** Either `admin_password` or `admin_ssh_key` must be specified.
    #[builder(into, default)]
    #[serde(rename = "disablePasswordAuthentication")]
    pub r#disable_password_authentication: Box<Option<bool>>,
    /// Specifies the mode of VM Guest Patching for the virtual machines that are associated to the Virtual Machine Scale Set. Possible values are `AutomaticByPlatform` or `ImageDefault`. Defaults to `ImageDefault`.
    /// 
    /// > **Note:** If the `patch_assessment_mode` is set to `AutomaticByPlatform` then the `provision_vm_agent` field must be set to `true`.
    #[builder(into, default)]
    #[serde(rename = "patchAssessmentMode")]
    pub r#patch_assessment_mode: Box<Option<String>>,
    /// Specifies the mode of in-guest patching of this Windows Virtual Machine. Possible values are `ImageDefault` or `AutomaticByPlatform`. Defaults to `ImageDefault`. For more information on patch modes please see the [product documentation](https://docs.microsoft.com/azure/virtual-machines/automatic-vm-guest-patching#patch-orchestration-modes).
    /// 
    /// > **Note:** If `patch_mode` is set to `AutomaticByPlatform` the `provision_vm_agent` must be set to `true` and the `extension` must contain at least one application health extension.  An example of how to correctly configure a Virtual Machine Scale Set to provision a Linux Virtual Machine with Automatic VM Guest Patching enabled can be found in the `./examples/orchestrated-vm-scale-set/automatic-vm-guest-patching` directory within the GitHub Repository.
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
    pub r#secrets: Box<Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileLinuxConfigurationSecret>>>,
}
