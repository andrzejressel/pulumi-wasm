#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddressIpTag {
    /// The IP Tag associated with the Public IP, such as `SQL` or `Storage`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
    /// The Type of IP Tag, such as `FirstPartyUsage`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
