#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceCapacityReservationSpecification {
    /// Indicates the instance's Capacity Reservation preferences. Can be `"open"` or `"none"`. (Default: `"open"`).
    #[builder(into, default)]
    #[serde(rename = "capacityReservationPreference")]
    pub r#capacity_reservation_preference: Box<Option<String>>,
    /// Information about the target Capacity Reservation. See Capacity Reservation Target below for more details.
    /// 
    /// For more information, see the documentation on [Capacity Reservations](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/capacity-reservations-using.html).
    #[builder(into, default)]
    #[serde(rename = "capacityReservationTarget")]
    pub r#capacity_reservation_target: Box<Option<super::super::types::ec2::InstanceCapacityReservationSpecificationCapacityReservationTarget>>,
}
