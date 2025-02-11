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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamArgs {
        /// Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes.
        #[builder(into, default)]
        pub cascade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A description for the IPAM.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enable this option to use your own GUA ranges as private IPv6 addresses. Default: `false`.
        #[builder(into, default)]
        pub enable_private_gua: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Determines which locales can be chosen when you create pools. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. You specify a region using the region_name parameter. You **must** set your provider block region as an operating_region.
        #[builder(into)]
        pub operating_regions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ec2::VpcIpamOperatingRegion>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// specifies the IPAM tier. Valid options include `free` and `advanced`. Default is `advanced`.
        #[builder(into, default)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamResult {
        /// Amazon Resource Name (ARN) of IPAM
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes.
        pub cascade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The IPAM's default resource discovery association ID.
        pub default_resource_discovery_association_id: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The IPAM's default resource discovery ID.
        pub default_resource_discovery_id: pulumi_gestalt_rust::Output<String>,
        /// A description for the IPAM.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enable this option to use your own GUA ranges as private IPv6 addresses. Default: `false`.
        pub enable_private_gua: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Determines which locales can be chosen when you create pools. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM. You can only create VPCs from a pool whose locale matches the VPC's Region. You specify a region using the region_name parameter. You **must** set your provider block region as an operating_region.
        pub operating_regions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::VpcIpamOperatingRegion>,
        >,
        /// The ID of the IPAM's private scope. A scope is a top-level container in IPAM. Each scope represents an IP-independent network. Scopes enable you to represent networks where you have overlapping IP space. When you create an IPAM, IPAM automatically creates two scopes: public and private. The private scope is intended for private IP space. The public scope is intended for all internet-routable IP space.
        pub private_default_scope_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the IPAM's public scope. A scope is a top-level container in IPAM. Each scope represents an IP-independent network. Scopes enable you to represent networks where you have overlapping IP space. When you create an IPAM, IPAM automatically creates two scopes: public and private. The private scope is intended for private
        /// IP space. The public scope is intended for all internet-routable IP space.
        pub public_default_scope_id: pulumi_gestalt_rust::Output<String>,
        /// The number of scopes in the IPAM.
        pub scope_count: pulumi_gestalt_rust::Output<i32>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// specifies the IPAM tier. Valid options include `free` and `advanced`. Default is `advanced`.
        pub tier: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamArgs,
    ) -> VpcIpamResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cascade_binding = args.cascade.get_output(context);
        let description_binding = args.description.get_output(context);
        let enable_private_gua_binding = args.enable_private_gua.get_output(context);
        let operating_regions_binding = args.operating_regions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tier_binding = args.tier.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpam:VpcIpam".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cascade".into(),
                    value: &cascade_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enablePrivateGua".into(),
                    value: &enable_private_gua_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operatingRegions".into(),
                    value: &operating_regions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tier".into(),
                    value: &tier_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamResult {
            arn: o.get_field("arn"),
            cascade: o.get_field("cascade"),
            default_resource_discovery_association_id: o
                .get_field("defaultResourceDiscoveryAssociationId"),
            default_resource_discovery_id: o.get_field("defaultResourceDiscoveryId"),
            description: o.get_field("description"),
            enable_private_gua: o.get_field("enablePrivateGua"),
            operating_regions: o.get_field("operatingRegions"),
            private_default_scope_id: o.get_field("privateDefaultScopeId"),
            public_default_scope_id: o.get_field("publicDefaultScopeId"),
            scope_count: o.get_field("scopeCount"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tier: o.get_field("tier"),
        }
    }
}
