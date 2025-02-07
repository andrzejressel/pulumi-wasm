pub mod get_vpc_iam_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcIamPoolArgs {
        /// Tags that are required to create resources in using this pool.
        #[builder(into, default)]
        pub allocation_resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolFilter>>,
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
    pub struct GetVpcIamPoolResult {
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
            Option<Vec<super::super::super::types::ec2::GetVpcIamPoolFilter>>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVpcIamPoolArgs,
    ) -> GetVpcIamPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allocation_resource_tags_binding = args
            .allocation_resource_tags
            .get_output(context)
            .get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let ipam_pool_id_binding = args.ipam_pool_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getVpcIamPool:getVpcIamPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationResourceTags".into(),
                    value: &allocation_resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "ipamPoolId".into(),
                    value: &ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVpcIamPoolResult {
            address_family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addressFamily"),
            ),
            allocation_default_netmask_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocationDefaultNetmaskLength"),
            ),
            allocation_max_netmask_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocationMaxNetmaskLength"),
            ),
            allocation_min_netmask_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocationMinNetmaskLength"),
            ),
            allocation_resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocationResourceTags"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_import: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoImport"),
            ),
            aws_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsService"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamPoolId"),
            ),
            ipam_scope_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamScopeId"),
            ),
            ipam_scope_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamScopeType"),
            ),
            locale: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locale"),
            ),
            pool_depth: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("poolDepth"),
            ),
            publicly_advertisable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publiclyAdvertisable"),
            ),
            source_ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceIpamPoolId"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
