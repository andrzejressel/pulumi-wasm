#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_vpc_ipam_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcIpamPoolArgs {
        /// Tags that are required to create resources in using this pool.
        #[builder(into, default)]
        pub allocation_resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcIpamPoolFilter>>,
        >,
        /// ID of the IPAM pool.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the IPAM pool you would like information on.
        #[builder(into, default)]
        pub ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcIpamPoolResult {
        /// IP protocol assigned to this pool.
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is `10.0.0.0/8` and you enter 16 here, new allocations will default to `10.0.0.0/16`.
        pub allocation_default_netmask_length: pulumi_gestalt_rust::Output<i32>,
        /// The maximum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_max_netmask_length: pulumi_gestalt_rust::Output<i32>,
        /// The minimum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_min_netmask_length: pulumi_gestalt_rust::Output<i32>,
        /// Tags that are required to create resources in using this pool.
        pub allocation_resource_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN of the pool
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// If enabled, IPAM will continuously look for resources within the CIDR range of this pool and automatically import them as allocations into your IPAM.
        pub auto_import: pulumi_gestalt_rust::Output<bool>,
        /// Limits which service in AWS that the pool can be used in. `ec2` for example, allows users to use space for Elastic IP addresses and VPCs.
        pub aws_service: pulumi_gestalt_rust::Output<String>,
        /// Description for the IPAM pool.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetVpcIpamPoolFilter>>,
        >,
        /// ID of the IPAM pool.
        pub id: pulumi_gestalt_rust::Output<Option<String>>,
        pub ipam_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the scope the pool belongs to.
        pub ipam_scope_id: pulumi_gestalt_rust::Output<String>,
        pub ipam_scope_type: pulumi_gestalt_rust::Output<String>,
        /// Locale is the Region where your pool is available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region.
        pub locale: pulumi_gestalt_rust::Output<String>,
        pub pool_depth: pulumi_gestalt_rust::Output<i32>,
        /// Defines whether or not IPv6 pool space is publicly advertisable over the internet.
        pub publicly_advertisable: pulumi_gestalt_rust::Output<bool>,
        /// ID of the source IPAM pool.
        pub source_ipam_pool_id: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVpcIpamPoolArgs,
    ) -> GetVpcIpamPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allocation_resource_tags_binding = args
            .allocation_resource_tags
            .get_output(context);
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getVpcIpamPool:getVpcIpamPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocationResourceTags".into(),
                    value: allocation_resource_tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamPoolId".into(),
                    value: ipam_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVpcIpamPoolResult {
            address_family: o.get_field("addressFamily"),
            allocation_default_netmask_length: o
                .get_field("allocationDefaultNetmaskLength"),
            allocation_max_netmask_length: o.get_field("allocationMaxNetmaskLength"),
            allocation_min_netmask_length: o.get_field("allocationMinNetmaskLength"),
            allocation_resource_tags: o.get_field("allocationResourceTags"),
            arn: o.get_field("arn"),
            auto_import: o.get_field("autoImport"),
            aws_service: o.get_field("awsService"),
            description: o.get_field("description"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ipam_pool_id: o.get_field("ipamPoolId"),
            ipam_scope_id: o.get_field("ipamScopeId"),
            ipam_scope_type: o.get_field("ipamScopeType"),
            locale: o.get_field("locale"),
            pool_depth: o.get_field("poolDepth"),
            publicly_advertisable: o.get_field("publiclyAdvertisable"),
            source_ipam_pool_id: o.get_field("sourceIpamPoolId"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
        }
    }
}
