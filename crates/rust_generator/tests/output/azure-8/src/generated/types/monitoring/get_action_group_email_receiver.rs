#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetActionGroupEmailReceiver {
    /// The email address of this receiver.
    #[builder(into)]
    #[serde(rename = "emailAddress")]
    pub r#email_address: Box<String>,
    /// Specifies the name of the Action Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Indicates whether to use common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Box<bool>,
}
