#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageRecipeBlockDeviceMapping {
    /// Name of the device. For example, `/dev/sda` or `/dev/xvdb`.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// Single list of object with Elastic Block Storage (EBS) block device mapping settings.
    #[builder(into)]
    #[serde(rename = "ebs")]
    pub r#ebs: Box<Vec<super::super::types::imagebuilder::GetImageRecipeBlockDeviceMappingEb>>,
    /// Whether to remove a mapping from the parent image.
    #[builder(into)]
    #[serde(rename = "noDevice")]
    pub r#no_device: Box<String>,
    /// Virtual device name. For example, `ephemeral0`. Instance store volumes are numbered starting from 0.
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Box<String>,
}