#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAmiBlockDeviceMapping {
    /// Physical name of the device.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// Map containing EBS information, if the device is EBS based. Unlike most object attributes, these are accessed directly (e.g., `ebs.volume_size` or `ebs["volume_size"]`) rather than accessed through the first element of a list (e.g., `ebs[0].volume_size`).
    #[builder(into)]
    #[serde(rename = "ebs")]
    pub r#ebs: Box<std::collections::HashMap<String, String>>,
    /// Suppresses the specified device included in the block device mapping of the AMI.
    #[builder(into)]
    #[serde(rename = "noDevice")]
    pub r#no_device: Box<String>,
    /// Virtual device name (for instance stores).
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Box<String>,
}
