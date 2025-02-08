#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfiguration {
    /// An `image` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfigurationImage>,
    /// An `os_profile` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "osProfile")]
    pub r#os_profile: Box<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfigurationOsProfile>,
    /// The size of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineSize")]
    pub r#virtual_machine_size: Box<String>,
}
