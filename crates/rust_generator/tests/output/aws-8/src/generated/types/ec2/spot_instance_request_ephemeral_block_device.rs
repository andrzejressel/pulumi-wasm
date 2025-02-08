#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpotInstanceRequestEphemeralBlockDevice {
    /// Name of the block device to mount on the instance.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// Suppresses the specified device included in the AMI's block device mapping.
    #[builder(into, default)]
    #[serde(rename = "noDevice")]
    pub r#no_device: Box<Option<bool>>,
    /// [Instance Store Device Name](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#InstanceStoreDeviceNames) (e.g., `ephemeral0`).
    /// 
    /// Each AWS Instance type has a different set of Instance Store block devices available for attachment. AWS [publishes a list](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/InstanceStorage.html#StorageOnInstanceTypes) of which ephemeral devices are available on each type. The devices are always identified by the `virtual_name` in the format `ephemeral{0..N}`.
    #[builder(into, default)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: Box<Option<String>>,
}
