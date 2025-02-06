#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceCapacityReservationSpecificationCapacityReservationTarget {
    /// ID of the Capacity Reservation in which to run the instance.
    #[builder(into, default)]
    #[serde(rename = "capacityReservationId")]
    pub r#capacity_reservation_id: Box<Option<String>>,
    /// ARN of the Capacity Reservation resource group in which to run the instance.
    #[builder(into, default)]
    #[serde(rename = "capacityReservationResourceGroupArn")]
    pub r#capacity_reservation_resource_group_arn: Box<Option<String>>,
}
