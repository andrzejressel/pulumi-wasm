#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineOsProfile {
    /// (Optional for Windows, Optional for Linux) The password associated with the local administrator account.
    /// 
    /// > **NOTE:** If using Linux, it may be preferable to use SSH Key authentication (available in the `os_profile_linux_config` block) instead of password authentication.
    #[builder(into, default)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: Box<Option<String>>,
    /// Specifies the name of the local administrator account.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: Box<String>,
    /// Specifies the name of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "computerName")]
    pub r#computer_name: Box<String>,
    /// Specifies custom data to supply to the machine. On Linux-based systems, this can be used as a cloud-init script. On other systems, this will be copied as a file on disk. Internally, this provider will base64 encode this value before sending it to the API. The maximum length of the binary array is 65535 bytes. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "customData")]
    pub r#custom_data: Box<Option<String>>,
}
