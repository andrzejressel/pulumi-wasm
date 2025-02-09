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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod reserved_cache_node {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservedCacheNodeArgs {
        /// Number of cache node instances to reserve.
        /// Default value is `1`.
        #[builder(into, default)]
        pub cache_node_count: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// ID of the reserved cache node offering to purchase.
        /// To determine an `reserved_cache_nodes_offering_id`, see the `aws.elasticache.getReservedCacheNodeOffering` data source.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub reserved_cache_nodes_offering_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticache::ReservedCacheNodeTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReservedCacheNodeResult {
        /// ARN for the reserved cache node.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Number of cache node instances to reserve.
        /// Default value is `1`.
        pub cache_node_count: pulumi_gestalt_rust::Output<f64>,
        /// Node type for the reserved cache nodes.
        pub cache_node_type: pulumi_gestalt_rust::Output<String>,
        /// Duration of the reservation as an RFC3339 duration.
        pub duration: pulumi_gestalt_rust::Output<String>,
        /// Fixed price charged for this reserved cache node.
        pub fixed_price: pulumi_gestalt_rust::Output<f64>,
        /// Offering type of this reserved cache node.
        pub offering_type: pulumi_gestalt_rust::Output<String>,
        /// Engine type for the reserved cache node.
        pub product_description: pulumi_gestalt_rust::Output<String>,
        /// Recurring price charged to run this reserved cache node.
        pub recurring_charges: pulumi_gestalt_rust::Output<
            Vec<super::super::types::elasticache::ReservedCacheNodeRecurringCharge>,
        >,
        /// ID of the reserved cache node offering to purchase.
        /// To determine an `reserved_cache_nodes_offering_id`, see the `aws.elasticache.getReservedCacheNodeOffering` data source.
        ///
        /// The following arguments are optional:
        pub reserved_cache_nodes_offering_id: pulumi_gestalt_rust::Output<String>,
        /// Time the reservation started.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// State of the reserved cache node.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticache::ReservedCacheNodeTimeouts>,
        >,
        /// Hourly price charged for this reserved cache node.
        pub usage_price: pulumi_gestalt_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReservedCacheNodeArgs,
    ) -> ReservedCacheNodeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cache_node_count_binding = args.cache_node_count.get_output(context);
        let reserved_cache_nodes_offering_id_binding = args
            .reserved_cache_nodes_offering_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticache/reservedCacheNode:ReservedCacheNode".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheNodeCount".into(),
                    value: cache_node_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservedCacheNodesOfferingId".into(),
                    value: reserved_cache_nodes_offering_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReservedCacheNodeResult {
            arn: o.get_field("arn"),
            cache_node_count: o.get_field("cacheNodeCount"),
            cache_node_type: o.get_field("cacheNodeType"),
            duration: o.get_field("duration"),
            fixed_price: o.get_field("fixedPrice"),
            offering_type: o.get_field("offeringType"),
            product_description: o.get_field("productDescription"),
            recurring_charges: o.get_field("recurringCharges"),
            reserved_cache_nodes_offering_id: o
                .get_field("reservedCacheNodesOfferingId"),
            start_time: o.get_field("startTime"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            usage_price: o.get_field("usagePrice"),
        }
    }
}
