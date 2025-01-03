pub mod get_instance_type_offerings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstanceTypeOfferings.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstanceTypeOfferingsFilter>>,
        >,
        /// Location type. Defaults to `region`. Valid values: `availability-zone`, `availability-zone-id`, and `region`.
        #[builder(into, default)]
        pub location_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstanceTypeOfferingsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of EC2 Instance Types.
        pub instance_types: pulumi_wasm_rust::Output<Vec<String>>,
        pub location_type: pulumi_wasm_rust::Output<Option<String>>,
        /// List of location types.
        pub location_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of locations.
        pub locations: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetInstanceTypeOfferingsArgs) -> GetInstanceTypeOfferingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let location_type_binding = args.location_type.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getInstanceTypeOfferings:getInstanceTypeOfferings".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "locationType".into(),
                    value: &location_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceTypes".into(),
                },
                register_interface::ResultField {
                    name: "locationType".into(),
                },
                register_interface::ResultField {
                    name: "locationTypes".into(),
                },
                register_interface::ResultField {
                    name: "locations".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceTypeOfferingsResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceTypes").unwrap(),
            ),
            location_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationType").unwrap(),
            ),
            location_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationTypes").unwrap(),
            ),
            locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locations").unwrap(),
            ),
        }
    }
}
