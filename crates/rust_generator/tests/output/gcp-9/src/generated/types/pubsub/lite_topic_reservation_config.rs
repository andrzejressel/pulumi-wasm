#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LiteTopicReservationConfig {
    /// The Reservation to use for this topic's throughput capacity.
    #[builder(into, default)]
    #[serde(rename = "throughputReservation")]
    pub r#throughput_reservation: Box<Option<String>>,
}
