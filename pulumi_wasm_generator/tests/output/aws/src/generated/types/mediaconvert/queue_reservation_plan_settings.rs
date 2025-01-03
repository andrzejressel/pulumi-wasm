#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QueueReservationPlanSettings {
    /// The length of the term of your reserved queue pricing plan commitment. Valid value is `ONE_YEAR`.
    #[builder(into)]
    #[serde(rename = "commitment")]
    pub r#commitment: Box<String>,
    /// Specifies whether the term of your reserved queue pricing plan. Valid values are `AUTO_RENEW` or `EXPIRE`.
    #[builder(into)]
    #[serde(rename = "renewalType")]
    pub r#renewal_type: Box<String>,
    /// Specifies the number of reserved transcode slots (RTS) for queue.
    #[builder(into)]
    #[serde(rename = "reservedSlots")]
    pub r#reserved_slots: Box<i32>,
}
