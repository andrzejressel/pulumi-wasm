pub mod get_capacity_block_offering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCapacityBlockOfferingArgs {
        /// The amount of time of the Capacity Block reservation in hours.
        #[builder(into)]
        pub capacity_duration_hours: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The date and time at which the Capacity Block Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub end_date_range: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The number of instances for which to reserve capacity.
        #[builder(into)]
        pub instance_count: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The instance type for which to reserve capacity.
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The date and time at which the Capacity Block Reservation starts. Valid values: [RFC3339 time string](https://tools.ietf.org/html/rfc3339#section-5.8) (`YYYY-MM-DDTHH:MM:SSZ`)
        #[builder(into, default)]
        pub start_date_range: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCapacityBlockOfferingResult {
        /// The Availability Zone in which to create the Capacity Reservation.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The Capacity Block Reservation ID.
        pub capacity_block_offering_id: pulumi_wasm_rust::Output<String>,
        pub capacity_duration_hours: pulumi_wasm_rust::Output<i32>,
        /// The currency of the payment for the Capacity Block.
        pub currency_code: pulumi_wasm_rust::Output<String>,
        pub end_date_range: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_count: pulumi_wasm_rust::Output<i32>,
        pub instance_type: pulumi_wasm_rust::Output<String>,
        pub start_date_range: pulumi_wasm_rust::Output<String>,
        /// Indicates the tenancy of the Capacity Reservation. Specify either `default` or `dedicated`.
        pub tenancy: pulumi_wasm_rust::Output<String>,
        /// The total price to be paid up front.
        pub upfront_fee: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCapacityBlockOfferingArgs,
    ) -> GetCapacityBlockOfferingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_duration_hours_binding = args
            .capacity_duration_hours
            .get_output(context)
            .get_inner();
        let end_date_range_binding = args.end_date_range.get_output(context).get_inner();
        let instance_count_binding = args.instance_count.get_output(context).get_inner();
        let instance_type_binding = args.instance_type.get_output(context).get_inner();
        let start_date_range_binding = args
            .start_date_range
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getCapacityBlockOffering:getCapacityBlockOffering".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityDurationHours".into(),
                    value: &capacity_duration_hours_binding,
                },
                register_interface::ObjectField {
                    name: "endDateRange".into(),
                    value: &end_date_range_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "startDateRange".into(),
                    value: &start_date_range_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "capacityBlockOfferingId".into(),
                },
                register_interface::ResultField {
                    name: "capacityDurationHours".into(),
                },
                register_interface::ResultField {
                    name: "currencyCode".into(),
                },
                register_interface::ResultField {
                    name: "endDateRange".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceCount".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "startDateRange".into(),
                },
                register_interface::ResultField {
                    name: "tenancy".into(),
                },
                register_interface::ResultField {
                    name: "upfrontFee".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCapacityBlockOfferingResult {
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            capacity_block_offering_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityBlockOfferingId").unwrap(),
            ),
            capacity_duration_hours: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityDurationHours").unwrap(),
            ),
            currency_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currencyCode").unwrap(),
            ),
            end_date_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endDateRange").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceCount").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            start_date_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startDateRange").unwrap(),
            ),
            tenancy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenancy").unwrap(),
            ),
            upfront_fee: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upfrontFee").unwrap(),
            ),
        }
    }
}
