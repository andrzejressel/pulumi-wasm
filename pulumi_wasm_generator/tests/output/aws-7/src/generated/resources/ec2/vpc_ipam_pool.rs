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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamPoolArgs {
        /// The IP protocol assigned to this pool. You must choose either IPv4 or IPv6 protocol for a pool.
        #[builder(into)]
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and you enter 16 here, new allocations will default to 10.0.0.0/16 (unless you provide a different netmask value when you create the new allocation).
        #[builder(into, default)]
        pub allocation_default_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum netmask length that will be required for CIDR allocations in this pool.
        #[builder(into, default)]
        pub allocation_max_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum netmask length that will be required for CIDR allocations in this pool.
        #[builder(into, default)]
        pub allocation_min_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// Tags that are required for resources that use CIDRs from this IPAM pool. Resources that do not have these tags will not be allowed to allocate space from the pool. If the resources have their tags changed after they have allocated space or if the allocation tagging requirements are changed on the pool, the resource may be marked as noncompliant.
        #[builder(into, default)]
        pub allocation_resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If you include this argument, IPAM automatically imports any VPCs you have in your scope that fall
        /// within the CIDR range in the pool.
        #[builder(into, default)]
        pub auto_import: pulumi_wasm_rust::Output<Option<bool>>,
        /// Limits which AWS service the pool can be used in. Only useable on public scopes. Valid Values: `ec2`.
        #[builder(into, default)]
        pub aws_service: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.
        #[builder(into, default)]
        pub cascade: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description for the IPAM pool.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the scope in which you would like to create the IPAM pool.
        #[builder(into)]
        pub ipam_scope_id: pulumi_wasm_rust::Output<String>,
        /// The locale in which you would like to create the IPAM pool. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. Possible values: Any AWS region, such as `us-east-1`.
        #[builder(into, default)]
        pub locale: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Valid values are `byoip` or `amazon`. Default is `byoip`.
        #[builder(into, default)]
        pub public_ip_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines whether or not IPv6 pool space is publicly advertisable over the internet. This argument is required if `address_family = "ipv6"` and `public_ip_source = "byoip"`, default is `false`. This option is not available for IPv4 pool space or if `public_ip_source = "amazon"`. Setting this argument to `true` when it is not available may result in erroneous differences being reported.
        #[builder(into, default)]
        pub publicly_advertisable: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the source IPAM pool. Use this argument to create a child pool within an existing pool.
        #[builder(into, default)]
        pub source_ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIpamPoolResult {
        /// The IP protocol assigned to this pool. You must choose either IPv4 or IPv6 protocol for a pool.
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// A default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and you enter 16 here, new allocations will default to 10.0.0.0/16 (unless you provide a different netmask value when you create the new allocation).
        pub allocation_default_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_max_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum netmask length that will be required for CIDR allocations in this pool.
        pub allocation_min_netmask_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// Tags that are required for resources that use CIDRs from this IPAM pool. Resources that do not have these tags will not be allowed to allocate space from the pool. If the resources have their tags changed after they have allocated space or if the allocation tagging requirements are changed on the pool, the resource may be marked as noncompliant.
        pub allocation_resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Amazon Resource Name (ARN) of IPAM
        pub arn: pulumi_wasm_rust::Output<String>,
        /// If you include this argument, IPAM automatically imports any VPCs you have in your scope that fall
        /// within the CIDR range in the pool.
        pub auto_import: pulumi_wasm_rust::Output<Option<bool>>,
        /// Limits which AWS service the pool can be used in. Only useable on public scopes. Valid Values: `ec2`.
        pub aws_service: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.
        pub cascade: pulumi_wasm_rust::Output<Option<bool>>,
        /// A description for the IPAM pool.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the scope in which you would like to create the IPAM pool.
        pub ipam_scope_id: pulumi_wasm_rust::Output<String>,
        pub ipam_scope_type: pulumi_wasm_rust::Output<String>,
        /// The locale in which you would like to create the IPAM pool. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. Possible values: Any AWS region, such as `us-east-1`.
        pub locale: pulumi_wasm_rust::Output<Option<String>>,
        pub pool_depth: pulumi_wasm_rust::Output<i32>,
        /// The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Valid values are `byoip` or `amazon`. Default is `byoip`.
        pub public_ip_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines whether or not IPv6 pool space is publicly advertisable over the internet. This argument is required if `address_family = "ipv6"` and `public_ip_source = "byoip"`, default is `false`. This option is not available for IPv4 pool space or if `public_ip_source = "amazon"`. Setting this argument to `true` when it is not available may result in erroneous differences being reported.
        pub publicly_advertisable: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the source IPAM pool. Use this argument to create a child pool within an existing pool.
        pub source_ipam_pool_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the IPAM
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcIpamPoolArgs) -> VpcIpamPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_family_binding = args.address_family.get_inner();
        let allocation_default_netmask_length_binding = args
            .allocation_default_netmask_length
            .get_inner();
        let allocation_max_netmask_length_binding = args
            .allocation_max_netmask_length
            .get_inner();
        let allocation_min_netmask_length_binding = args
            .allocation_min_netmask_length
            .get_inner();
        let allocation_resource_tags_binding = args.allocation_resource_tags.get_inner();
        let auto_import_binding = args.auto_import.get_inner();
        let aws_service_binding = args.aws_service.get_inner();
        let cascade_binding = args.cascade.get_inner();
        let description_binding = args.description.get_inner();
        let ipam_scope_id_binding = args.ipam_scope_id.get_inner();
        let locale_binding = args.locale.get_inner();
        let public_ip_source_binding = args.public_ip_source.get_inner();
        let publicly_advertisable_binding = args.publicly_advertisable.get_inner();
        let source_ipam_pool_id_binding = args.source_ipam_pool_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamPool:VpcIpamPool".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressFamily".into(),
                },
                register_interface::ResultField {
                    name: "allocationDefaultNetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "allocationMaxNetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "allocationMinNetmaskLength".into(),
                },
                register_interface::ResultField {
                    name: "allocationResourceTags".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoImport".into(),
                },
                register_interface::ResultField {
                    name: "awsService".into(),
                },
                register_interface::ResultField {
                    name: "cascade".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "ipamScopeId".into(),
                },
                register_interface::ResultField {
                    name: "ipamScopeType".into(),
                },
                register_interface::ResultField {
                    name: "locale".into(),
                },
                register_interface::ResultField {
                    name: "poolDepth".into(),
                },
                register_interface::ResultField {
                    name: "publicIpSource".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAdvertisable".into(),
                },
                register_interface::ResultField {
                    name: "sourceIpamPoolId".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcIpamPoolResult {
            address_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressFamily").unwrap(),
            ),
            allocation_default_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationDefaultNetmaskLength").unwrap(),
            ),
            allocation_max_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationMaxNetmaskLength").unwrap(),
            ),
            allocation_min_netmask_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationMinNetmaskLength").unwrap(),
            ),
            allocation_resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationResourceTags").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_import: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoImport").unwrap(),
            ),
            aws_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsService").unwrap(),
            ),
            cascade: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cascade").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            ipam_scope_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamScopeId").unwrap(),
            ),
            ipam_scope_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamScopeType").unwrap(),
            ),
            locale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locale").unwrap(),
            ),
            pool_depth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolDepth").unwrap(),
            ),
            public_ip_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpSource").unwrap(),
            ),
            publicly_advertisable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAdvertisable").unwrap(),
            ),
            source_ipam_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceIpamPoolId").unwrap(),
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
        }
    }
}
