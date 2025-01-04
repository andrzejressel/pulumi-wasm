pub mod get_spot_price {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSpotPriceArgs {
        /// Availability zone in which to query Spot price information.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSpotPriceHistory.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetSpotPriceFilter>>,
        >,
        /// Type of instance for which to query Spot Price information.
        #[builder(into, default)]
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSpotPriceResult {
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetSpotPriceFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Most recent Spot Price value for the given instance type and AZ.
        pub spot_price: pulumi_wasm_rust::Output<String>,
        /// The timestamp at which the Spot Price value was published.
        pub spot_price_timestamp: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSpotPriceArgs) -> GetSpotPriceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zone_binding = args.availability_zone.get_inner();
        let filters_binding = args.filters.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getSpotPrice:getSpotPrice".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "spotPrice".into(),
                },
                register_interface::ResultField {
                    name: "spotPriceTimestamp".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSpotPriceResult {
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            spot_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spotPrice").unwrap(),
            ),
            spot_price_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spotPriceTimestamp").unwrap(),
            ),
        }
    }
}
