#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_capacity_block_offering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCapacityBlockOfferingArgs {
        /// The amount of time of the Capacity Block reservation in hours.
        #[builder(into)]
        pub capacity_duration_hours: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The date and time at which the Capacity Block Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub end_date_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of instances for which to reserve capacity.
        #[builder(into)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The instance type for which to reserve capacity.
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The date and time at which the Capacity Block Reservation starts. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub start_date_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCapacityBlockOfferingResult {
        /// The Availability Zone in which to create the Capacity Reservation.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The Capacity Block Reservation ID.
        pub capacity_block_offering_id: pulumi_gestalt_rust::Output<String>,
        pub capacity_duration_hours: pulumi_gestalt_rust::Output<i32>,
        /// The currency of the payment for the Capacity Block.
        pub currency_code: pulumi_gestalt_rust::Output<String>,
        pub end_date_range: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_count: pulumi_gestalt_rust::Output<i32>,
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        pub start_date_range: pulumi_gestalt_rust::Output<String>,
        /// Indicates the tenancy of the Capacity Reservation. Specify either `default` or `dedicated`.
        pub tenancy: pulumi_gestalt_rust::Output<String>,
        /// The total price to be paid up front.
        pub upfront_fee: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCapacityBlockOfferingArgs,
    ) -> GetCapacityBlockOfferingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_duration_hours_binding = args
            .capacity_duration_hours
            .get_output(context);
        let end_date_range_binding = args.end_date_range.get_output(context);
        let instance_count_binding = args.instance_count.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let start_date_range_binding = args.start_date_range.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getCapacityBlockOffering:getCapacityBlockOffering".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityDurationHours".into(),
                    value: capacity_duration_hours_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endDateRange".into(),
                    value: end_date_range_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: instance_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startDateRange".into(),
                    value: start_date_range_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCapacityBlockOfferingResult {
            availability_zone: o.get_field("availabilityZone"),
            capacity_block_offering_id: o.get_field("capacityBlockOfferingId"),
            capacity_duration_hours: o.get_field("capacityDurationHours"),
            currency_code: o.get_field("currencyCode"),
            end_date_range: o.get_field("endDateRange"),
            id: o.get_field("id"),
            instance_count: o.get_field("instanceCount"),
            instance_type: o.get_field("instanceType"),
            start_date_range: o.get_field("startDateRange"),
            tenancy: o.get_field("tenancy"),
            upfront_fee: o.get_field("upfrontFee"),
        }
    }
}
