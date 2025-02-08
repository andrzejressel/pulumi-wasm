#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessIdentityProviderScimConfig {
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "groupMemberDeprovision")]
    pub r#group_member_deprovision: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "seatDeprovision")]
    pub r#seat_deprovision: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "secret")]
    pub r#secret: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "userDeprovision")]
    pub r#user_deprovision: Box<Option<bool>>,
}
