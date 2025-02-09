#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_public_ipv_4_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicIpv4PoolArgs {
        /// AWS resource IDs of a public IPv4 pool (as a string) for which this data source will fetch detailed information.
        #[builder(into)]
        pub pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Any tags for the address pool.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPublicIpv4PoolResult {
        /// Description of the pool, if any.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the location from which the address pool is advertised.
        /// * pool_address_ranges` - List of Address Ranges in the Pool; each address range record contains:
        pub network_border_group: pulumi_gestalt_rust::Output<String>,
        pub pool_address_ranges: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetPublicIpv4PoolPoolAddressRange>,
        >,
        pub pool_id: pulumi_gestalt_rust::Output<String>,
        /// Any tags for the address pool.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Total number of addresses in the pool.
        pub total_address_count: pulumi_gestalt_rust::Output<i32>,
        /// Total number of available addresses in the pool.
        pub total_available_address_count: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPublicIpv4PoolArgs,
    ) -> GetPublicIpv4PoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let pool_id_binding = args.pool_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getPublicIpv4Pool:getPublicIpv4Pool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolId".into(),
                    value: pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPublicIpv4PoolResult {
            description: o.get_field("description"),
            id: o.get_field("id"),
            network_border_group: o.get_field("networkBorderGroup"),
            pool_address_ranges: o.get_field("poolAddressRanges"),
            pool_id: o.get_field("poolId"),
            tags: o.get_field("tags"),
            total_address_count: o.get_field("totalAddressCount"),
            total_available_address_count: o.get_field("totalAvailableAddressCount"),
        }
    }
}
