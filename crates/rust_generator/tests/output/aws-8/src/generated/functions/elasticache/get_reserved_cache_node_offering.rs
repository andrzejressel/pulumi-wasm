#[allow(clippy::doc_lazy_continuation)]
pub mod get_reserved_cache_node_offering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReservedCacheNodeOfferingArgs {
        /// Node type for the reserved cache node.
        /// See AWS documentation for information on [supported node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Redis](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/nodes-select-size.html).
        /// See AWS documentation for information on [supported node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/CacheNodes.SupportedTypes.html) and [guidance on selecting node types for Memcached](https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/nodes-select-size.html).
        #[builder(into)]
        pub cache_node_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Duration of the reservation in RFC3339 duration format.
        /// Valid values are `P1Y` (one year) and `P3Y` (three years).
        #[builder(into)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Offering type of this reserved cache node.
        /// For the latest generation of nodes (e.g. M5, R5, T4 and newer) valid values are `No Upfront`, `Partial Upfront`, and `All Upfront`.
        /// For other current generation nodes (i.e. T2, M3, M4, R3, or R4) the only valid value is `Heavy Utilization`.
        /// For previous generation modes (i.e. T1, M1, M2, or C1) valid values are `Heavy Utilization`, `Medium Utilization`, and `Light Utilization`.
        #[builder(into)]
        pub offering_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Engine type for the reserved cache node.
        /// Valid values are `redis`, `valkey` and `memcached`.
        #[builder(into)]
        pub product_description: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReservedCacheNodeOfferingResult {
        pub cache_node_type: pulumi_gestalt_rust::Output<String>,
        pub duration: pulumi_gestalt_rust::Output<String>,
        /// Fixed price charged for this reserved cache node.
        pub fixed_price: pulumi_gestalt_rust::Output<f64>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the reservation.
        pub offering_id: pulumi_gestalt_rust::Output<String>,
        pub offering_type: pulumi_gestalt_rust::Output<String>,
        pub product_description: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetReservedCacheNodeOfferingArgs,
    ) -> GetReservedCacheNodeOfferingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cache_node_type_binding = args
            .cache_node_type
            .get_output(context)
            .get_inner();
        let duration_binding = args.duration.get_output(context).get_inner();
        let offering_type_binding = args.offering_type.get_output(context).get_inner();
        let product_description_binding = args
            .product_description
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReservedCacheNodeOfferingResult {
            cache_node_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheNodeType"),
            ),
            duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("duration"),
            ),
            fixed_price: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fixedPrice"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            offering_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("offeringId"),
            ),
            offering_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("offeringType"),
            ),
            product_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productDescription"),
            ),
        }
    }
}
