/// Manages an ElastiCache Reserved Cache Node.
///
/// > **NOTE:** Once created, a reservation is valid for the `duration` of the provided `offering_id` and cannot be deleted. Performing a `destroy` will only remove the resource from state. For more information see [ElastiCache Reserved Nodes Documentation](https://aws.amazon.com/elasticache/reserved-cache-nodes/) and [PurchaseReservedCacheNodesOffering](https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_PurchaseReservedCacheNodesOffering.html).
///
/// > **NOTE:** Due to the expense of testing this resource, we provide it as best effort. If you find it useful, and have the ability to help test or notice issues, consider reaching out to us on GitHub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleReservedCacheNode:
///     type: aws:elasticache:ReservedCacheNode
///     name: example
///     properties:
///       reservedCacheNodesOfferingId: ${example.offeringId}
///       id: optionalCustomReservationID
///       cacheNodeCount: 3
/// variables:
///   example:
///     fn::invoke:
///       function: aws:elasticache:getReservedCacheNodeOffering
///       arguments:
///         cacheNodeType: cache.t4g.small
///         duration: P1Y
///         offeringType: No Upfront
///         productDescription: redis
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache Reserved Cache Node using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/reservedCacheNode:ReservedCacheNode example CustomReservationID
/// ```
pub mod reserved_cache_node {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservedCacheNodeArgs {
        /// Number of cache node instances to reserve.
        /// Default value is `1`.
        #[builder(into, default)]
        pub cache_node_count: pulumi_wasm_rust::Output<Option<f64>>,
        /// ID of the reserved cache node offering to purchase.
        /// To determine an `reserved_cache_nodes_offering_id`, see the `aws.elasticache.getReservedCacheNodeOffering` data source.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub reserved_cache_nodes_offering_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticache::ReservedCacheNodeTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReservedCacheNodeResult {
        /// ARN for the reserved cache node.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Number of cache node instances to reserve.
        /// Default value is `1`.
        pub cache_node_count: pulumi_wasm_rust::Output<f64>,
        /// Node type for the reserved cache nodes.
        pub cache_node_type: pulumi_wasm_rust::Output<String>,
        /// Duration of the reservation as an RFC3339 duration.
        pub duration: pulumi_wasm_rust::Output<String>,
        /// Fixed price charged for this reserved cache node.
        pub fixed_price: pulumi_wasm_rust::Output<f64>,
        /// Offering type of this reserved cache node.
        pub offering_type: pulumi_wasm_rust::Output<String>,
        /// Engine type for the reserved cache node.
        pub product_description: pulumi_wasm_rust::Output<String>,
        /// Recurring price charged to run this reserved cache node.
        pub recurring_charges: pulumi_wasm_rust::Output<
            Vec<super::super::types::elasticache::ReservedCacheNodeRecurringCharge>,
        >,
        /// ID of the reserved cache node offering to purchase.
        /// To determine an `reserved_cache_nodes_offering_id`, see the `aws.elasticache.getReservedCacheNodeOffering` data source.
        ///
        /// The following arguments are optional:
        pub reserved_cache_nodes_offering_id: pulumi_wasm_rust::Output<String>,
        /// Time the reservation started.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// State of the reserved cache node.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticache::ReservedCacheNodeTimeouts>,
        >,
        /// Hourly price charged for this reserved cache node.
        pub usage_price: pulumi_wasm_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ReservedCacheNodeArgs) -> ReservedCacheNodeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cache_node_count_binding = args.cache_node_count.get_inner();
        let reserved_cache_nodes_offering_id_binding = args
            .reserved_cache_nodes_offering_id
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/reservedCacheNode:ReservedCacheNode".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cacheNodeCount".into(),
                    value: &cache_node_count_binding,
                },
                register_interface::ObjectField {
                    name: "reservedCacheNodesOfferingId".into(),
                    value: &reserved_cache_nodes_offering_id_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cacheNodeCount".into(),
                },
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
                    name: "offeringType".into(),
                },
                register_interface::ResultField {
                    name: "productDescription".into(),
                },
                register_interface::ResultField {
                    name: "recurringCharges".into(),
                },
                register_interface::ResultField {
                    name: "reservedCacheNodesOfferingId".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "usagePrice".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReservedCacheNodeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cache_node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheNodeCount").unwrap(),
            ),
            cache_node_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheNodeType").unwrap(),
            ),
            duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("duration").unwrap(),
            ),
            fixed_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fixedPrice").unwrap(),
            ),
            offering_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offeringType").unwrap(),
            ),
            product_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productDescription").unwrap(),
            ),
            recurring_charges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurringCharges").unwrap(),
            ),
            reserved_cache_nodes_offering_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservedCacheNodesOfferingId").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            usage_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usagePrice").unwrap(),
            ),
        }
    }
}
