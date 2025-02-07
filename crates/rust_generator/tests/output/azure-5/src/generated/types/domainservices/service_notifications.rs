#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceNotifications {
    /// A list of additional email addresses to notify when there are alerts in the managed domain.
    #[builder(into, default)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Box<Option<Vec<String>>>,
    /// Whether to notify members of the _AAD DC Administrators_ group when there are alerts in the managed domain.
    #[builder(into, default)]
    #[serde(rename = "notifyDcAdmins")]
    pub r#notify_dc_admins: Box<Option<bool>>,
    /// Whether to notify all Global Administrators when there are alerts in the managed domain.
    #[builder(into, default)]
    #[serde(rename = "notifyGlobalAdmins")]
    pub r#notify_global_admins: Box<Option<bool>>,
}
