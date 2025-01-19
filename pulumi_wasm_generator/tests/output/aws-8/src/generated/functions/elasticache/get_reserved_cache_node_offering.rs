pub mod get_reserved_cache_node_offering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReservedCacheNodeOfferingArgs {
        /// Node type for the reserved cache node.
        /// See AWS documentation for information on [supported node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// See AWS documentation for information on [supported node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/nodes-select-size.html).
        #[builder(into)]
        pub cache_node_type: pulumi_wasm_rust::Output<String>,
        /// Duration of the reservation in RFC3339 duration format.
        /// Valid values are `P1Y` (one year) and `P3Y` (three years).
        #[builder(into)]
        pub duration: pulumi_wasm_rust::Output<String>,
        /// Offering type of this reserved cache node.
        /// For the latest generation of nodes (e.g. M5, R5, T4 and newer) valid values are `No Upfront`, `Partial Upfront`, and `All Upfront`.
        /// For other current generation nodes (i.e. T2, M3, M4, R3, or R4) the only valid value is `Heavy Utilization`.
        /// For previous generation modes (i.e. T1, M1, M2, or C1) valid values are `Heavy Utilization`, `Medium Utilization`, and `Light Utilization`.
        #[builder(into)]
        pub offering_type: pulumi_wasm_rust::Output<String>,
        /// Engine type for the reserved cache node.
        /// Valid values are `redis`, `valkey` and `memcached`.
        #[builder(into)]
        pub product_description: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetReservedCacheNodeOfferingResult {
        pub cache_node_type: pulumi_wasm_rust::Output<String>,
        pub duration: pulumi_wasm_rust::Output<String>,
        /// Fixed price charged for this reserved cache node.
        pub fixed_price: pulumi_wasm_rust::Output<f64>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the reservation.
        pub offering_id: pulumi_wasm_rust::Output<String>,
        pub offering_type: pulumi_wasm_rust::Output<String>,
        pub product_description: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetReservedCacheNodeOfferingArgs,
    ) -> GetReservedCacheNodeOfferingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cache_node_type_binding = args.cache_node_type.get_inner();
        let duration_binding = args.duration.get_inner();
        let offering_type_binding = args.offering_type.get_inner();
        let product_description_binding = args.product_description.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticache/getReservedCacheNodeOffering:getReservedCacheNodeOffering"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cacheNodeType".into(),
                    value: &cache_node_type_binding,
                },
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "offeringType".into(),
                    value: &offering_type_binding,
                },
                register_interface::ObjectField {
                    name: "productDescription".into(),
                    value: &product_description_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cacheNodeType".into(),
                },
                register_interface::ResultField {
                    name: "duration".into(),
                },
                register_interface::ResultField {
                    name: "fixedPrice".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "offeringId".into(),
                },
                register_interface::ResultField {
                    name: "offeringType".into(),
                },
                register_interface::ResultField {
                    name: "productDescription".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReservedCacheNodeOfferingResult {
            cache_node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheNodeType").unwrap(),
            ),
            duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("duration").unwrap(),
            ),
            fixed_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fixedPrice").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            offering_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offeringId").unwrap(),
            ),
            offering_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offeringType").unwrap(),
            ),
            product_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productDescription").unwrap(),
            ),
        }
    }
}
