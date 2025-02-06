#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScaleSetOsProfile {
    /// Specifies the administrator password to use for all the instances of virtual machines in a scale set.
    #[builder(into, default)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: Box<Option<String>>,
    /// Specifies the administrator account name to use for all the instances of virtual machines in the scale set.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: Box<String>,
    /// Specifies the computer name prefix for all of the virtual machines in the scale set. Computer name prefixes must be 1 to 9 characters long for windows images and 1 - 58 for Linux. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "computerNamePrefix")]
    pub r#computer_name_prefix: Box<String>,
    /// Specifies custom data to supply to the machine. On Linux-based systems, this can be used as a cloud-init script. On other systems, this will be copied as a file on disk. Internally, this provider will base64 encode this value before sending it to the API. The maximum length of the binary array is 65535 bytes.
    #[builder(into, default)]
    #[serde(rename = "customData")]
    pub r#custom_data: Box<Option<String>>,
}
