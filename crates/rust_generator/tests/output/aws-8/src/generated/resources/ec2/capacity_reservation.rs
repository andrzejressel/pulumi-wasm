/// Provides an EC2 Capacity Reservation. This allows you to reserve capacity for your Amazon EC2 instances in a specific Availability Zone for any duration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod capacity_reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityReservationArgs {
        /// The Availability Zone in which to create the Capacity Reservation.
        #[builder(into)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates whether the Capacity Reservation supports EBS-optimized instances.
        #[builder(into, default)]
        pub ebs_optimized: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub end_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates the way in which the Capacity Reservation ends. Specify either `unlimited` or `limited`.
        #[builder(into, default)]
        pub end_date_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates whether the Capacity Reservation supports instances with temporary, block-level storage.
        #[builder(into, default)]
        pub ephemeral_storage: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The number of instances for which to reserve capacity.
        #[builder(into)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Indicates the type of instance launches that the Capacity Reservation accepts. Specify either `open` or `targeted`.
        #[builder(into, default)]
        pub instance_match_criteria: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of operating system for which to reserve capacity. Valid options are `Linux/UNIX`, `Red Hat Enterprise Linux`, `SUSE Linux`, `Windows`, `Windows with SQL Server`, `Windows with SQL Server Enterprise`, `Windows with SQL Server Standard` or `Windows with SQL Server Web`.
        #[builder(into)]
        pub instance_platform: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The instance type for which to reserve capacity.
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity Reservation.
        #[builder(into, default)]
        pub outpost_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the cluster placement group in which to create the Capacity Reservation.
        #[builder(into, default)]
        pub placement_group_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates the tenancy of the Capacity Reservation. Specify either `default` or `dedicated`.
        #[builder(into, default)]
        pub tenancy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CapacityReservationResult {
        /// The ARN of the Capacity Reservation.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Availability Zone in which to create the Capacity Reservation.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the Capacity Reservation supports EBS-optimized instances.
        pub ebs_optimized: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        pub end_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates the way in which the Capacity Reservation ends. Specify either `unlimited` or `limited`.
        pub end_date_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether the Capacity Reservation supports instances with temporary, block-level storage.
        pub ephemeral_storage: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The number of instances for which to reserve capacity.
        pub instance_count: pulumi_gestalt_rust::Output<i32>,
        /// Indicates the type of instance launches that the Capacity Reservation accepts. Specify either `open` or `targeted`.
        pub instance_match_criteria: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of operating system for which to reserve capacity. Valid options are `Linux/UNIX`, `Red Hat Enterprise Linux`, `SUSE Linux`, `Windows`, `Windows with SQL Server`, `Windows with SQL Server Enterprise`, `Windows with SQL Server Standard` or `Windows with SQL Server Web`.
        pub instance_platform: pulumi_gestalt_rust::Output<String>,
        /// The instance type for which to reserve capacity.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Outpost on which to create the Capacity Reservation.
        pub outpost_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the AWS account that owns the Capacity Reservation.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the cluster placement group in which to create the Capacity Reservation.
        pub placement_group_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Indicates the tenancy of the Capacity Reservation. Specify either `default` or `dedicated`.
        pub tenancy: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CapacityReservationArgs,
    ) -> CapacityReservationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zone_binding = args.availability_zone.get_output(context);
        let ebs_optimized_binding = args.ebs_optimized.get_output(context);
        let end_date_binding = args.end_date.get_output(context);
        let end_date_type_binding = args.end_date_type.get_output(context);
        let ephemeral_storage_binding = args.ephemeral_storage.get_output(context);
        let instance_count_binding = args.instance_count.get_output(context);
        let instance_match_criteria_binding = args
            .instance_match_criteria
            .get_output(context);
        let instance_platform_binding = args.instance_platform.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let outpost_arn_binding = args.outpost_arn.get_output(context);
        let placement_group_arn_binding = args.placement_group_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tenancy_binding = args.tenancy.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/capacityReservation:CapacityReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ebsOptimized".into(),
                    value: &ebs_optimized_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endDate".into(),
                    value: &end_date_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endDateType".into(),
                    value: &end_date_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ephemeralStorage".into(),
                    value: &ephemeral_storage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceMatchCriteria".into(),
                    value: &instance_match_criteria_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instancePlatform".into(),
                    value: &instance_platform_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outpostArn".into(),
                    value: &outpost_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placementGroupArn".into(),
                    value: &placement_group_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenancy".into(),
                    value: &tenancy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CapacityReservationResult {
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            ebs_optimized: o.get_field("ebsOptimized"),
            end_date: o.get_field("endDate"),
            end_date_type: o.get_field("endDateType"),
            ephemeral_storage: o.get_field("ephemeralStorage"),
            instance_count: o.get_field("instanceCount"),
            instance_match_criteria: o.get_field("instanceMatchCriteria"),
            instance_platform: o.get_field("instancePlatform"),
            instance_type: o.get_field("instanceType"),
            outpost_arn: o.get_field("outpostArn"),
            owner_id: o.get_field("ownerId"),
            placement_group_arn: o.get_field("placementGroupArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tenancy: o.get_field("tenancy"),
        }
    }
}
