/// Provides an IPAM Resource Discovery resource. IPAM Resource Discoveries are resources meant for multi-organization customers. If you wish to use a single IPAM across multiple orgs, a resource discovery can be created and shared from a subordinate organization to the management organizations IPAM delegated admin account. For a full deployment example, see `aws.ec2.VpcIpamResourceDiscoveryAssociation` resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:VpcIpamResourceDiscovery
///     properties:
///       description: My IPAM Resource Discovery
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
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the IPAM resource discovery `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamResourceDiscovery:VpcIpamResourceDiscovery example ipam-res-disco-0178368ad2146a492
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_resource_discovery {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamResourceDiscoveryArgs {
        /// A description for the IPAM Resource Discovery.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Determines which regions the Resource Discovery will enable IPAM features for usage and monitoring. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM Resource Discovery. You can only create VPCs from a pool whose locale matches the VPC's Region. You specify a region using the region_name parameter. **You must set your provider block region as an operating_region.**
        #[builder(into)]
        pub operating_regions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ec2::VpcIpamResourceDiscoveryOperatingRegion>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIpamResourceDiscoveryResult {
        /// Amazon Resource Name (ARN) of IPAM Resource Discovery
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description for the IPAM Resource Discovery.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The home region of the Resource Discovery
        pub ipam_resource_discovery_region: pulumi_gestalt_rust::Output<String>,
        /// A boolean to identify if the Resource Discovery is the accounts default resource discovery
        pub is_default: pulumi_gestalt_rust::Output<bool>,
        /// Determines which regions the Resource Discovery will enable IPAM features for usage and monitoring. Locale is the Region where you want to make an IPAM pool available for allocations. You can only create pools with locales that match the operating Regions of the IPAM Resource Discovery. You can only create VPCs from a pool whose locale matches the VPC's Region. You specify a region using the region_name parameter. **You must set your provider block region as an operating_region.**
        pub operating_regions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::VpcIpamResourceDiscoveryOperatingRegion>,
        >,
        /// The account ID for the account that manages the Resource Discovery
        pub owner_id: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamResourceDiscoveryArgs,
    ) -> VpcIpamResourceDiscoveryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let operating_regions_binding = args.operating_regions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamResourceDiscovery:VpcIpamResourceDiscovery".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operatingRegions".into(),
                    value: &operating_regions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamResourceDiscoveryResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            ipam_resource_discovery_region: o.get_field("ipamResourceDiscoveryRegion"),
            is_default: o.get_field("isDefault"),
            operating_regions: o.get_field("operatingRegions"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
