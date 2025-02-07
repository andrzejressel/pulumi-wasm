#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageRecipeBlockDeviceMapping {
    /// Name of the device. For example, `/dev/sda` or `/dev/xvdb`.
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// Configuration block with Elastic Block Storage (EBS) block device mapping settings. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "ebs")]
    pub r#ebs: Box<Option<super::super::types::imagebuilder::ImageRecipeBlockDeviceMappingEbs>>,
    /// Set to `true` to remove a mapping from the parent image.
    #[builder(into, default)]
    #[serde(rename = "noDevice")]
    pub r#no_device: Box<Option<bool>>,
    /// Virtual device name. For example, `ephemeral0`. Instance store volumes are numbered starting from 0.
    #[builder(into, default)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Box<Option<String>>,
}
