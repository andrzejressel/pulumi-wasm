#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrchestratedVirtualMachineScaleSetOsProfileWindowsConfigurationAdditionalUnattendContent {
    /// The XML formatted content that is added to the unattend.xml file for the specified path and component. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// The name of the setting to which the content applies. Possible values are `AutoLogon` and `FirstLogonCommands`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "setting")]
    pub r#setting: Box<String>,
}
