#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationInstallPatch {
    /// A `linux` block as defined below.
    #[builder(into)]
    #[serde(rename = "linuxes")]
    pub r#linuxes: Box<Vec<super::super::types::maintenance::GetConfigurationInstallPatchLinux>>,
    /// Possible reboot preference as defined by the user based on which it would be decided to reboot the machine or not after the patch operation is completed.
    #[builder(into)]
    #[serde(rename = "reboot")]
    pub r#reboot: Box<String>,
    /// A `windows` block as defined below.
    #[builder(into)]
    #[serde(rename = "windows")]
    pub r#windows: Box<Vec<super::super::types::maintenance::GetConfigurationInstallPatchWindow>>,
}
