#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPhoneConfig {
    /// The After Call Work (ACW) timeout setting, in seconds.
    #[builder(into)]
    #[serde(rename = "afterContactWorkTimeLimit")]
    pub r#after_contact_work_time_limit: Box<i32>,
    /// When Auto-Accept Call is enabled for an available agent, the agent connects to contacts automatically.
    #[builder(into)]
    #[serde(rename = "autoAccept")]
    pub r#auto_accept: Box<bool>,
    /// The phone number for the user's desk phone.
    #[builder(into)]
    #[serde(rename = "deskPhoneNumber")]
    pub r#desk_phone_number: Box<String>,
    /// The phone type. Valid values are `DESK_PHONE` and `SOFT_PHONE`.
    #[builder(into)]
    #[serde(rename = "phoneType")]
    pub r#phone_type: Box<String>,
}
