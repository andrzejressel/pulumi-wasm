#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDirectoryWorkspaceAccessProperty {
    /// (Optional) Indicates whether users can use Android devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeAndroid")]
    pub r#device_type_android: Box<String>,
    /// (Optional) Indicates whether users can use Chromebooks to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeChromeos")]
    pub r#device_type_chromeos: Box<String>,
    /// (Optional) Indicates whether users can use iOS devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeIos")]
    pub r#device_type_ios: Box<String>,
    /// (Optional) Indicates whether users can use Linux clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeLinux")]
    pub r#device_type_linux: Box<String>,
    /// (Optional) Indicates whether users can use macOS clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeOsx")]
    pub r#device_type_osx: Box<String>,
    /// (Optional) Indicates whether users can access their WorkSpaces through a web browser.
    #[builder(into)]
    #[serde(rename = "deviceTypeWeb")]
    pub r#device_type_web: Box<String>,
    /// (Optional) Indicates whether users can use Windows clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeWindows")]
    pub r#device_type_windows: Box<String>,
    /// (Optional) Indicates whether users can use zero client devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeZeroclient")]
    pub r#device_type_zeroclient: Box<String>,
}
