#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrchestratedVirtualMachineScaleSetOsProfile {
    /// The Base64-Encoded Custom Data which should be used for this Virtual Machine Scale Set.
    /// 
    /// > **Note:** When Custom Data has been configured, it's not possible to remove it without tainting the Virtual Machine Scale Set, due to a limitation of the Azure API.
    #[builder(into, default)]
    #[serde(rename = "customData")]
    pub r#custom_data: Box<Option<String>>,
    /// A `linux_configuration` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "linuxConfiguration")]
    pub r#linux_configuration: Box<Option<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileLinuxConfiguration>>,
    /// A `windows_configuration` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "windowsConfiguration")]
    pub r#windows_configuration: Box<Option<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsProfileWindowsConfiguration>>,
}
