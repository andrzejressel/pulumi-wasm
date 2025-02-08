#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitorUser {
    /// Email of the user used by Datadog for contacting them if needed. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// The name which should be used for this user_info. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Phone number of the user used by Datadog for contacting them if needed. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<Option<String>>,
}
