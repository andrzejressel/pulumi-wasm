#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetOnDemandOptions {
    /// The order of the launch template overrides to use in fulfilling On-Demand capacity. Valid values: `lowestPrice`, `prioritized`. Default: `lowestPrice`.
    #[builder(into, default)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Box<Option<String>>,
    /// The strategy for using unused Capacity Reservations for fulfilling On-Demand capacity. Supported only for fleets of type `instant`.
    #[builder(into, default)]
    #[serde(rename = "capacityReservationOptions")]
    pub r#capacity_reservation_options: Box<Option<super::super::types::ec2::FleetOnDemandOptionsCapacityReservationOptions>>,
    /// The maximum amount per hour for On-Demand Instances that you're willing to pay.
    #[builder(into, default)]
    #[serde(rename = "maxTotalPrice")]
    pub r#max_total_price: Box<Option<String>>,
    /// The minimum target capacity for On-Demand Instances in the fleet. If the minimum target capacity is not reached, the fleet launches no instances. Supported only for fleets of type `instant`.
    /// If you specify `min_target_capacity`, at least one of the following must be specified: `single_availability_zone` or `single_instance_type`.
    #[builder(into, default)]
    #[serde(rename = "minTargetCapacity")]
    pub r#min_target_capacity: Box<Option<i32>>,
    /// Indicates that the fleet launches all On-Demand Instances into a single Availability Zone. Supported only for fleets of type `instant`.
    #[builder(into, default)]
    #[serde(rename = "singleAvailabilityZone")]
    pub r#single_availability_zone: Box<Option<bool>>,
    /// Indicates that the fleet uses a single instance type to launch all On-Demand Instances in the fleet. Supported only for fleets of type `instant`.
    #[builder(into, default)]
    #[serde(rename = "singleInstanceType")]
    pub r#single_instance_type: Box<Option<bool>>,
}
