#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NextGenerationFirewallVirtualHubPanoramaPanorama {
    #[builder(into, default)]
    #[serde(rename = "deviceGroupName")]
    pub r#device_group_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "panoramaServer1")]
    pub r#panorama_server_1: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "panoramaServer2")]
    pub r#panorama_server_2: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "templateName")]
    pub r#template_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "virtualMachineSshKey")]
    pub r#virtual_machine_ssh_key: Box<Option<String>>,
}
