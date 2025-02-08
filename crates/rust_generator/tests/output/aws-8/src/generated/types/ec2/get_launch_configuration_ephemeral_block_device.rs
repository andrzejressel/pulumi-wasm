#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchConfigurationEphemeralBlockDevice {
    /// Name of the device.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// Virtual Name of the device.
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Box<String>,
}
