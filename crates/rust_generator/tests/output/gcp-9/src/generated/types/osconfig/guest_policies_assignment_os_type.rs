#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuestPoliciesAssignmentOsType {
    /// Targets VM instances with OS Inventory enabled and having the following OS architecture.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "osArchitecture")]
    pub r#os_architecture: Box<Option<String>>,
    /// Targets VM instances with OS Inventory enabled and having the following OS short name, for example "debian" or "windows".
    #[builder(into, default)]
    #[serde(rename = "osShortName")]
    pub r#os_short_name: Box<Option<String>>,
    /// Targets VM instances with OS Inventory enabled and having the following following OS version.
    #[builder(into, default)]
    #[serde(rename = "osVersion")]
    pub r#os_version: Box<Option<String>>,
}
