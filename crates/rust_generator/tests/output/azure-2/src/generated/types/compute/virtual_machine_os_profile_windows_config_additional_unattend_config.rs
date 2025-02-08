#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineOsProfileWindowsConfigAdditionalUnattendConfig {
    /// Specifies the name of the component to configure with the added content. The only allowable value is `Microsoft-Windows-Shell-Setup`.
    #[builder(into)]
    #[serde(rename = "component")]
    pub r#component: Box<String>,
    /// Specifies the base-64 encoded XML formatted content that is added to the unattend.xml file for the specified path and component.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// Specifies the name of the pass that the content applies to. The only allowable value is `oobeSystem`.
    #[builder(into)]
    #[serde(rename = "pass")]
    pub r#pass: Box<String>,
    /// Specifies the name of the setting to which the content applies. Possible values are: `FirstLogonCommands` and `AutoLogon`.
    #[builder(into)]
    #[serde(rename = "settingName")]
    pub r#setting_name: Box<String>,
}
