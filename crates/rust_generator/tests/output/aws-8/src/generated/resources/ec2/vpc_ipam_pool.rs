/// Provides an IP address pool resource for IPAM.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpam
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
///   exampleVpcIpamPool:
///     type: aws:ec2:VpcIpamPool
///     name: example
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${example.privateDefaultScopeId}
///       locale: ${current.name}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// Nested Pools:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpam
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
///   parent:
///     type: aws:ec2:VpcIpamPool
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${example.privateDefaultScopeId}
///   parentTest:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: parent_test
///     properties:
///       ipamPoolId: ${parent.id}
///       cidr: 172.20.0.0/16
///   child:
///     type: aws:ec2:VpcIpamPool
///     properties:
///       addressFamily: ipv4
///       ipamScopeId: ${example.privateDefaultScopeId}
///       locale: ${current.name}
///       sourceIpamPoolId: ${parent.id}
///   childTest:
///     type: aws:ec2:VpcIpamPoolCidr
///     name: child_test
///     properties:
///       ipamPoolId: ${child.id}
///       cidr: 172.20.0.0/24
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the IPAM pool `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamPool:VpcIpamPool example ipam-pool-0958f95207d978e1e
/// ```
pub mod vpc_ipam_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPoolArgs {
        /// The IP protocol assigned to this pool. You must choose either IPv4 or IPv6 protocol for a pool.
        #[builder(into)]
        pub address_family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and you enter 16 here, new allocations will default to 10.0.0.0/16 (unless you provide a different netmask value when you create the new allocation).
        #[builder(into, default)]
        pub allocation_default_netmask_length: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The maximum netmask length that will be required for CIDR allocations in this pool.
        #[builder(into, default)]
        pub allocation_max_netmask_length: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The minimum netmask length that will be required for CIDR allocations in this pool.
        #[builder(into, default)]
        pub allocation_min_netmask_length: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Tags that are required for resources that use CIDRs from this IPAM pool. Resources that do not have these tags will not be allowed to allocate space from the pool. If the resources have their tags changed after they have allocated space or if the allocation tagging requirements are changed on the pool, the resource may be marked as noncompliant.
        #[builder(into, default)]
        pub allocation_resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If you include this argument, IPAM automatically imports any VPCs you have in your scope that fall
        /// within the CIDR range in the pool.
        #[builder(into, default)]
        pub auto_import: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Limits which AWS service the pool can be used in. Only useable on public scopes. Valid Values: `ec2`.
        #[builder(into, default)]
        pub aws_service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.
        #[builder(into, default)]
        pub cascade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description for the IPAM pool.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the scope in which you would like to create the IPAM pool.
        #[builder(into)]
        pub ipam_scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The locale in which you would like to create the IPAM pool. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. Possible values: Any AWS region, such as `us-east-1`.
        #[builder(into, default)]
        pub locale: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Valid values are `byoip` or `amazon`. Default is `byoip`.
        #[builder(into, default)]
        pub public_ip_source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines whether or not IPv6 pool space is publicly advertisable over the internet. This argument is required if `address_family = "ipv6"` and `public_ip_source = "byoip"`, default is `false`. This option is not available for IPv4 pool space or if `public_ip_source = "amazon"`. Setting this argument to `true` when it is not available may result in erroneous differences being reported.
        #[builder(into, default)]
        pub publicly_advertisable: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the source IPAM pool. Use this argument to create a child pool within an existing pool.
        #[builder(into, default)]
        pub source_ipam_pool_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPoolResult {
        /// The IP protocol assigned to this pool. You must choose either IPv4 or IPv6 protocol for a pool.
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and you enter 16 here, new allocations will default to 10.0.0.0/16 (unless you provide a different netmask value when you create the new allocation).
        pub allocation_default_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_max_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The minimum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_min_netmask_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Tags that are required for resources that use CIDRs from this IPAM pool. Resources that do not have these tags will not be allowed to allocate space from the pool. If the resources have their tags changed after they have allocated space or if the allocation tagging requirements are changed on the pool, the resource may be marked as noncompliant.
        pub allocation_resource_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Amazon Resource Name (ARN) of IPAM
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// If you include this argument, IPAM automatically imports any VPCs you have in your scope that fall
        /// within the CIDR range in the pool.
        pub auto_import: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Limits which AWS service the pool can be used in. Only useable on public scopes. Valid Values: `ec2`.
        pub aws_service: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.
        pub cascade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A description for the IPAM pool.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the scope in which you would like to create the IPAM pool.
        pub ipam_scope_id: pulumi_gestalt_rust::Output<String>,
        pub ipam_scope_type: pulumi_gestalt_rust::Output<String>,
        /// The locale in which you would like to create the IPAM pool. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. Possible values: Any AWS region, such as `us-east-1`.
        pub locale: pulumi_gestalt_rust::Output<Option<String>>,
        pub pool_depth: pulumi_gestalt_rust::Output<i32>,
        /// The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Valid values are `byoip` or `amazon`. Default is `byoip`.
        pub public_ip_source: pulumi_gestalt_rust::Output<Option<String>>,
        /// Defines whether or not IPv6 pool space is publicly advertisable over the internet. This argument is required if `address_family = "ipv6"` and `public_ip_source = "byoip"`, default is `false`. This option is not available for IPv4 pool space or if `public_ip_source = "amazon"`. Setting this argument to `true` when it is not available may result in erroneous differences being reported.
        pub publicly_advertisable: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the source IPAM pool. Use this argument to create a child pool within an existing pool.
        pub source_ipam_pool_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the IPAM
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcIpamPoolArgs,
    ) -> VpcIpamPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let address_family_binding = args.address_family.get_output(context).get_inner();
        let allocation_default_netmask_length_binding = args
            .allocation_default_netmask_length
            .get_output(context)
            .get_inner();
        let allocation_max_netmask_length_binding = args
            .allocation_max_netmask_length
            .get_output(context)
            .get_inner();
        let allocation_min_netmask_length_binding = args
            .allocation_min_netmask_length
            .get_output(context)
            .get_inner();
        let allocation_resource_tags_binding = args
            .allocation_resource_tags
            .get_output(context)
            .get_inner();
        let auto_import_binding = args.auto_import.get_output(context).get_inner();
        let aws_service_binding = args.aws_service.get_output(context).get_inner();
        let cascade_binding = args.cascade.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let ipam_scope_id_binding = args.ipam_scope_id.get_output(context).get_inner();
        let locale_binding = args.locale.get_output(context).get_inner();
        let public_ip_source_binding = args
            .public_ip_source
            .get_output(context)
            .get_inner();
        let publicly_advertisable_binding = args
            .publicly_advertisable
            .get_output(context)
            .get_inner();
        let source_ipam_pool_id_binding = args
            .source_ipam_pool_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPool:VpcIpamPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressFamily".into(),
                    value: &address_family_binding,
                },
                register_interface::ObjectField {
                    name: "allocationDefaultNetmaskLength".into(),
                    value: &allocation_default_netmask_length_binding,
                },
                register_interface::ObjectField {
                    name: "allocationMaxNetmaskLength".into(),
                    value: &allocation_max_netmask_length_binding,
                },
                register_interface::ObjectField {
                    name: "allocationMinNetmaskLength".into(),
                    value: &allocation_min_netmask_length_binding,
                },
                register_interface::ObjectField {
                    name: "allocationResourceTags".into(),
                    value: &allocation_resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "autoImport".into(),
                    value: &auto_import_binding,
                },
                register_interface::ObjectField {
                    name: "awsService".into(),
                    value: &aws_service_binding,
                },
                register_interface::ObjectField {
                    name: "cascade".into(),
                    value: &cascade_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipamScopeId".into(),
                    value: &ipam_scope_id_binding,
                },
                register_interface::ObjectField {
                    name: "locale".into(),
                    value: &locale_binding,
                },
                register_interface::ObjectField {
                    name: "publicIpSource".into(),
                    value: &public_ip_source_binding,
                },
                register_interface::ObjectField {
                    name: "publiclyAdvertisable".into(),
                    value: &publicly_advertisable_binding,
                },
                register_interface::ObjectField {
                    name: "sourceIpamPoolId".into(),
                    value: &source_ipam_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcIpamPoolResult {
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
            cascade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cascade"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
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
            public_ip_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpSource"),
            ),
            publicly_advertisable: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publiclyAdvertisable"),
            ),
            source_ipam_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceIpamPoolId"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
