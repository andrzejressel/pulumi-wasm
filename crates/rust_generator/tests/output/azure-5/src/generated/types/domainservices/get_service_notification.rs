#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceNotification {
    /// A list of additional email addresses to notify when there are alerts in the managed domain.
    #[builder(into)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Box<Vec<String>>,
    /// Whethermembers of the _AAD DC Administrators_ group are notified when there are alerts in the managed domain.
    #[builder(into)]
    #[serde(rename = "notifyDcAdmins")]
    pub r#notify_dc_admins: Box<bool>,
    /// Whether all Global Administrators are notified when there are alerts in the managed domain.
    #[builder(into)]
    #[serde(rename = "notifyGlobalAdmins")]
    pub r#notify_global_admins: Box<bool>,
}
