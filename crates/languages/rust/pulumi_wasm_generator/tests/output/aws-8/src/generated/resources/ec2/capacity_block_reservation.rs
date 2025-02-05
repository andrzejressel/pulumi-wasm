/// Provides an EC2 Capacity Block Reservation. This allows you to purchase capacity block for your Amazon EC2 instances in a specific Availability Zone for machine learning (ML) Workloads.
///
/// > **NOTE:** Once created, a reservation is valid for the `duration` of the provided `capacity_block_offering_id` and cannot be deleted. Performing a `destroy` will only remove the resource from state. For more information see [EC2 Capacity Block Reservation Documentation](https://aws.amazon.com/ec2/instance-types/p5/) and [PurchaseReservedDBInstancesOffering](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/capacity-blocks-pricing-billing.html).
///
/// > **NOTE:** Due to the expense of testing this resource, we provide it as best effort. If you find it useful, and have the ability to help test or notice issues, consider reaching out to us on GitHub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:CapacityBlockReservation
///     properties:
///       capacityBlockOfferingId: ${test.capacityBlockOfferingId}
///       instancePlatform: Linux/UNIX
///       tags:
///         Environment: dev
/// variables:
///   test:
///     fn::invoke:
///       function: aws:ec2:getCapacityBlockOffering
///       arguments:
///         capacityDurationHours: 24
///         endDateRange: 2024-05-30T15:04:05Z
///         instanceCount: 1
///         instanceType: p4d.24xlarge
///         startDateRange: 2024-04-28T15:04:05Z
/// ```
pub mod capacity_block_reservation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityBlockReservationArgs {
        /// The Capacity Block Reservation ID.
        #[builder(into)]
        pub capacity_block_offering_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of operating system for which to reserve capacity. Valid options are `Linux/UNIX`, `Red Hat Enterprise Linux`, `SUSE Linux`, `Windows`, `Windows with SQL Server`, `Windows with SQL Server Enterprise`, `Windows with SQL Server Standard` or `Windows with SQL Server Web`.
        #[builder(into)]
        pub instance_platform: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::CapacityBlockReservationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct CapacityBlockReservationResult {
        /// The ARN of the reservation.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone in which to create the Capacity Block Reservation.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The Capacity Block Reservation ID.
        pub capacity_block_offering_id: pulumi_wasm_rust::Output<String>,
        /// The date and time at which the Capacity Block Reservation was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the Capacity Reservation supports EBS-optimized instances.
        pub ebs_optimized: pulumi_wasm_rust::Output<bool>,
        /// The date and time at which the Capacity Block Reservation expires. When a Capacity Block Reservation expires, the reserved capacity is released and you can no longer launch instances into it. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub end_date: pulumi_wasm_rust::Output<String>,
        /// Indicates the way in which the Capacity Reservation ends.
        pub end_date_type: pulumi_wasm_rust::Output<String>,
        /// The number of instances for which to reserve capacity.
        pub instance_count: pulumi_wasm_rust::Output<i32>,
        /// The type of operating system for which to reserve capacity. Valid options are `Linux/UNIX`, `Red Hat Enterprise Linux`, `SUSE Linux`, `Windows`, `Windows with SQL Server`, `Windows with SQL Server Enterprise`, `Windows with SQL Server Standard` or `Windows with SQL Server Web`.
        pub instance_platform: pulumi_wasm_rust::Output<String>,
        /// The instance type for which to reserve capacity.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The ARN of the Outpost on which to create the Capacity Block Reservation.
        pub outpost_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the placement group in which to create the Capacity Block Reservation.
        pub placement_group_arn: pulumi_wasm_rust::Output<String>,
        /// The type of Capacity Reservation.
        pub reservation_type: pulumi_wasm_rust::Output<String>,
        /// The date and time at which the Capacity Block Reservation starts. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub start_date: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates the tenancy of the Capacity Block Reservation. Specify either `default` or `dedicated`.
        pub tenancy: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::CapacityBlockReservationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CapacityBlockReservationArgs,
    ) -> CapacityBlockReservationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_block_offering_id_binding = args
            .capacity_block_offering_id
            .get_output(context)
            .get_inner();
        let instance_platform_binding = args
            .instance_platform
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/capacityBlockReservation:CapacityBlockReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityBlockOfferingId".into(),
                    value: &capacity_block_offering_id_binding,
                },
                register_interface::ObjectField {
                    name: "instancePlatform".into(),
                    value: &instance_platform_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CapacityBlockReservationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            capacity_block_offering_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityBlockOfferingId"),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ebsOptimized"),
            ),
            end_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endDate"),
            ),
            end_date_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endDateType"),
            ),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceCount"),
            ),
            instance_platform: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instancePlatform"),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outpostArn"),
            ),
            placement_group_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("placementGroupArn"),
            ),
            reservation_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reservationType"),
            ),
            start_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startDate"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tenancy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenancy"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
