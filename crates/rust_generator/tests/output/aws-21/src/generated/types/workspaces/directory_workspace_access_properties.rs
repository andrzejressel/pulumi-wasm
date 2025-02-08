#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DirectoryWorkspaceAccessProperties {
    /// Indicates whether users can use Android devices to access their WorkSpaces.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeAndroid")]
    pub r#device_type_android: Box<Option<String>>,
    /// Indicates whether users can use Chromebooks to access their WorkSpaces.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeChromeos")]
    pub r#device_type_chromeos: Box<Option<String>>,
    /// Indicates whether users can use iOS devices to access their WorkSpaces.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeIos")]
    pub r#device_type_ios: Box<Option<String>>,
    /// Indicates whether users can use Linux clients to access their WorkSpaces.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeLinux")]
    pub r#device_type_linux: Box<Option<String>>,
    /// Indicates whether users can use macOS clients to access their WorkSpaces.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeOsx")]
    pub r#device_type_osx: Box<Option<String>>,
    /// Indicates whether users can access their WorkSpaces through a web browser.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeWeb")]
    pub r#device_type_web: Box<Option<String>>,
    /// Indicates whether users can use Windows clients to access their WorkSpaces.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeWindows")]
    pub r#device_type_windows: Box<Option<String>>,
    /// Indicates whether users can use zero client devices to access their WorkSpaces.
    #[builder(into, default)]
    #[serde(rename = "deviceTypeZeroclient")]
    pub r#device_type_zeroclient: Box<Option<String>>,
}
