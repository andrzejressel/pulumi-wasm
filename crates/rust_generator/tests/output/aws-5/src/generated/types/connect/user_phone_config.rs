#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPhoneConfig {
    /// The After Call Work (ACW) timeout setting, in seconds. Minimum value of 0.
    #[builder(into, default)]
    #[serde(rename = "afterContactWorkTimeLimit")]
    pub r#after_contact_work_time_limit: Box<Option<i32>>,
    /// When Auto-Accept Call is enabled for an available agent, the agent connects to contacts automatically.
    #[builder(into, default)]
    #[serde(rename = "autoAccept")]
    pub r#auto_accept: Box<Option<bool>>,
    /// The phone number for the user's desk phone. Required if `phone_type` is set as `DESK_PHONE`.
    #[builder(into, default)]
    #[serde(rename = "deskPhoneNumber")]
    pub r#desk_phone_number: Box<Option<String>>,
    /// The phone type. Valid values are `DESK_PHONE` and `SOFT_PHONE`.
    #[builder(into)]
    #[serde(rename = "phoneType")]
    pub r#phone_type: Box<String>,
}
