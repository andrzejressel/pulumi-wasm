/// Provides an IPAM resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:VpcIpam
///     properties:
///       description: My IPAM
///       operatingRegions:
///         - regionName: ${current.name}
///       tags:
///         Test: Main
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// Shared with multiple operating_regions:
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the IPAM `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpam:VpcIpam example ipam-0178368ad2146a492
/// ```
pub mod vpc_ipam {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamArgs {
        /// Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes.
        #[builder(into, default)]
        pub cascade: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A description for the IPAM.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enable this option to use your own GUA ranges as private IPv6 addresses. Default: `false`.
        #[builder(into, default)]
        pub enable_private_gua: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Determines which locales can be chosen when you create pools. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. You specify a region using the region_name parameter. You **must** set your provider block region as an operating_region.
        #[builder(into)]
        pub operating_regions: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::ec2::VpcIpamOperatingRegion>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// specifies the IPAM tier. Valid options include `free` and `advanced`. Default is `advanced`.
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamResult {
        /// Amazon Resource Name (ARN) of IPAM
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes.
        pub cascade: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IPAM's default resource discovery association ID.
        pub default_resource_discovery_association_id: pulumi_wasm_rust::Output<String>,
        /// The IPAM's default resource discovery ID.
        pub default_resource_discovery_id: pulumi_wasm_rust::Output<String>,
        /// A description for the IPAM.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable this option to use your own GUA ranges as private IPv6 addresses. Default: `false`.
        pub enable_private_gua: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines which locales can be chosen when you create pools. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. You specify a region using the region_name parameter. You **must** set your provider block region as an operating_region.
        pub operating_regions: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::VpcIpamOperatingRegion>,
        >,
        /// The ID of the IPAM's private scope. A scope is a top-level container in IPAM. Each scope represents an IP-independent network. Scopes enable you to represent networks where you have overlapping IP space. When you create an IPAM, IPAM automatically creates two scopes: public and private. The private scope is intended for private IP space. The public scope is intended for all internet-routable IP space.
        pub private_default_scope_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the IPAM's public scope. A scope is a top-level container in IPAM. Each scope represents an IP-independent network. Scopes enable you to represent networks where you have overlapping IP space. When you create an IPAM, IPAM automatically creates two scopes: public and private. The private scope is intended for private
        /// IP space. The public scope is intended for all internet-routable IP space.
        pub public_default_scope_id: pulumi_wasm_rust::Output<String>,
        /// The number of scopes in the IPAM.
        pub scope_count: pulumi_wasm_rust::Output<i32>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// specifies the IPAM tier. Valid options include `free` and `advanced`. Default is `advanced`.
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcIpamArgs,
    ) -> VpcIpamResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cascade_binding = args.cascade.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enable_private_gua_binding = args
            .enable_private_gua
            .get_output(context)
            .get_inner();
        let operating_regions_binding = args
            .operating_regions
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tier_binding = args.tier.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpam:VpcIpam".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cascade".into(),
                    value: &cascade_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enablePrivateGua".into(),
                    value: &enable_private_gua_binding,
                },
                register_interface::ObjectField {
                    name: "operatingRegions".into(),
                    value: &operating_regions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcIpamResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cascade: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cascade"),
            ),
            default_resource_discovery_association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultResourceDiscoveryAssociationId"),
            ),
            default_resource_discovery_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultResourceDiscoveryId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enable_private_gua: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enablePrivateGua"),
            ),
            operating_regions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("operatingRegions"),
            ),
            private_default_scope_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateDefaultScopeId"),
            ),
            public_default_scope_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicDefaultScopeId"),
            ),
            scope_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scopeCount"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(o.extract_field("tier")),
        }
    }
}
