/// Provides an EC2 Capacity Reservation. This allows you to reserve capacity for your Amazon EC2 instances in a specific Availability Zone for any duration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = capacity_reservation::create(
///         "default",
///         CapacityReservationArgs::builder()
///             .availability_zone("eu-west-1a")
///             .instance_count(1)
///             .instance_platform("Linux/UNIX")
///             .instance_type("t2.micro")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Capacity Reservations using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/capacityReservation:CapacityReservation web cr-0123456789abcdef0
/// ```
pub mod capacity_reservation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityReservationArgs {
        /// The Availability Zone in which to create the Capacity Reservation.
        #[builder(into)]
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the Capacity Reservation supports EBS-optimized instances.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<bool>>,
        /// The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub end_date: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates the way in which the Capacity Reservation ends. Specify either `unlimited` or `limited`.
        #[builder(into, default)]
        pub end_date_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the Capacity Reservation supports instances with temporary, block-level storage.
        #[builder(into, default)]
        pub ephemeral_storage: pulumi_wasm_rust::Output<Option<bool>>,
        /// The number of instances for which to reserve capacity.
        #[builder(into)]
        pub instance_count: pulumi_wasm_rust::Output<i32>,
        /// Indicates the type of instance launches that the Capacity Reservation accepts. Specify either `open` or `targeted`.
        #[builder(into, default)]
        pub instance_match_criteria: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of operating system for which to reserve capacity. Valid options are `Linux/UNIX`, `Red Hat Enterprise Linux`, `SUSE Linux`, `Windows`, `Windows with SQL Server`, `Windows with SQL Server Enterprise`, `Windows with SQL Server Standard` or `Windows with SQL Server Web`.
        #[builder(into)]
        pub instance_platform: pulumi_wasm_rust::Output<String>,
        /// The instance type for which to reserve capacity.
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity Reservation.
        #[builder(into, default)]
        pub outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the cluster placement group in which to create the Capacity Reservation.
        #[builder(into, default)]
        pub placement_group_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates the tenancy of the Capacity Reservation. Specify either `default` or `dedicated`.
        #[builder(into, default)]
        pub tenancy: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CapacityReservationResult {
        /// The ARN of the Capacity Reservation.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone in which to create the Capacity Reservation.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the Capacity Reservation supports EBS-optimized instances.
        pub ebs_optimized: pulumi_wasm_rust::Output<Option<bool>>,
        /// The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub end_date: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates the way in which the Capacity Reservation ends. Specify either `unlimited` or `limited`.
        pub end_date_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates whether the Capacity Reservation supports instances with temporary, block-level storage.
        pub ephemeral_storage: pulumi_wasm_rust::Output<Option<bool>>,
        /// The number of instances for which to reserve capacity.
        pub instance_count: pulumi_wasm_rust::Output<i32>,
        /// Indicates the type of instance launches that the Capacity Reservation accepts. Specify either `open` or `targeted`.
        pub instance_match_criteria: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of operating system for which to reserve capacity. Valid options are `Linux/UNIX`, `Red Hat Enterprise Linux`, `SUSE Linux`, `Windows`, `Windows with SQL Server`, `Windows with SQL Server Enterprise`, `Windows with SQL Server Standard` or `Windows with SQL Server Web`.
        pub instance_platform: pulumi_wasm_rust::Output<String>,
        /// The instance type for which to reserve capacity.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity Reservation.
        pub outpost_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the AWS account that owns the Capacity Reservation.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the cluster placement group in which to create the Capacity Reservation.
        pub placement_group_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates the tenancy of the Capacity Reservation. Specify either `default` or `dedicated`.
        pub tenancy: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CapacityReservationArgs,
    ) -> CapacityReservationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zone_binding = args.availability_zone.get_inner();
        let ebs_optimized_binding = args.ebs_optimized.get_inner();
        let end_date_binding = args.end_date.get_inner();
        let end_date_type_binding = args.end_date_type.get_inner();
        let ephemeral_storage_binding = args.ephemeral_storage.get_inner();
        let instance_count_binding = args.instance_count.get_inner();
        let instance_match_criteria_binding = args.instance_match_criteria.get_inner();
        let instance_platform_binding = args.instance_platform.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let outpost_arn_binding = args.outpost_arn.get_inner();
        let placement_group_arn_binding = args.placement_group_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let tenancy_binding = args.tenancy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/capacityReservation:CapacityReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding,
                },
                register_interface::ObjectField {
                    name: "endDate".into(),
                    value: &end_date_binding,
                },
                register_interface::ObjectField {
                    name: "endDateType".into(),
                    value: &end_date_type_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralStorage".into(),
                    value: &ephemeral_storage_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "instanceMatchCriteria".into(),
                    value: &instance_match_criteria_binding,
                },
                register_interface::ObjectField {
                    name: "instancePlatform".into(),
                    value: &instance_platform_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "outpostArn".into(),
                    value: &outpost_arn_binding,
                },
                register_interface::ObjectField {
                    name: "placementGroupArn".into(),
                    value: &placement_group_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tenancy".into(),
                    value: &tenancy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptimized".into(),
                },
                register_interface::ResultField {
                    name: "endDate".into(),
                },
                register_interface::ResultField {
                    name: "endDateType".into(),
                },
                register_interface::ResultField {
                    name: "ephemeralStorage".into(),
                },
                register_interface::ResultField {
                    name: "instanceCount".into(),
                },
                register_interface::ResultField {
                    name: "instanceMatchCriteria".into(),
                },
                register_interface::ResultField {
                    name: "instancePlatform".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "outpostArn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "placementGroupArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tenancy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CapacityReservationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            ebs_optimized: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptimized").unwrap(),
            ),
            end_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endDate").unwrap(),
            ),
            end_date_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endDateType").unwrap(),
            ),
            ephemeral_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ephemeralStorage").unwrap(),
            ),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceCount").unwrap(),
            ),
            instance_match_criteria: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceMatchCriteria").unwrap(),
            ),
            instance_platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instancePlatform").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            outpost_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostArn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            placement_group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementGroupArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenancy").unwrap(),
            ),
        }
    }
}
