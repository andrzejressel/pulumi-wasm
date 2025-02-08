#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[builder(into, default)]
    #[serde(rename = "disable")]
    pub r#disable: Box<Option<bool>>,
    /// SELinux level label
    #[builder(into, default)]
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// SELinux role label
    #[builder(into, default)]
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
    /// SELinux type label
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// SELinux user label
    #[builder(into, default)]
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}
