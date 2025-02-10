/// Manages a VPN Site.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-vwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVpnSite = vpn_site::create(
///         "exampleVpnSite",
///         VpnSiteArgs::builder()
///             .address_cidrs(vec!["10.0.0.0/24",])
///             .links(
///                 vec![
///                     VpnSiteLink::builder().ipAddress("10.0.0.1").name("link1")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("site1")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// VPN Sites can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/vpnSite:VpnSite example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/vpnSites/site1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpn_site {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnSiteArgs {
        /// Specifies a list of IP address CIDRs that are located on your on-premises site. Traffic destined for these address spaces is routed to your local site.
        ///
        /// > **NOTE:** The `address_cidrs` has to be set when the `link.bgp` isn't specified.
        #[builder(into, default)]
        pub address_cidrs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The model of the VPN device.
        #[builder(into, default)]
        pub device_model: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the VPN device vendor.
        #[builder(into, default)]
        pub device_vendor: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `link` blocks as defined below.
        #[builder(into, default)]
        pub links: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::VpnSiteLink>>,
        >,
        /// The Azure Region where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this VPN Site. Changing this forces a new VPN Site to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `o365_policy` block as defined below.
        #[builder(into, default)]
        pub o365_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VpnSiteO365Policy>,
        >,
        /// The name of the Resource Group where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the VPN Site.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Wan where this VPN site resides in. Changing this forces a new VPN Site to be created.
        #[builder(into)]
        pub virtual_wan_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnSiteResult {
        /// Specifies a list of IP address CIDRs that are located on your on-premises site. Traffic destined for these address spaces is routed to your local site.
        ///
        /// > **NOTE:** The `address_cidrs` has to be set when the `link.bgp` isn't specified.
        pub address_cidrs: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The model of the VPN device.
        pub device_model: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the VPN device vendor.
        pub device_vendor: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `link` blocks as defined below.
        pub links: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::VpnSiteLink>>,
        >,
        /// The Azure Region where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this VPN Site. Changing this forces a new VPN Site to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `o365_policy` block as defined below.
        pub o365_policy: pulumi_gestalt_rust::Output<
            super::super::types::network::VpnSiteO365Policy,
        >,
        /// The name of the Resource Group where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the VPN Site.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Wan where this VPN site resides in. Changing this forces a new VPN Site to be created.
        pub virtual_wan_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnSiteArgs,
    ) -> VpnSiteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_cidrs_binding = args.address_cidrs.get_output(context);
        let device_model_binding = args.device_model.get_output(context);
        let device_vendor_binding = args.device_vendor.get_output(context);
        let links_binding = args.links.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let o365_policy_binding = args.o365_policy.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_wan_id_binding = args.virtual_wan_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/vpnSite:VpnSite".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressCidrs".into(),
                    value: address_cidrs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceModel".into(),
                    value: device_model_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceVendor".into(),
                    value: device_vendor_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "links".into(),
                    value: links_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "o365Policy".into(),
                    value: o365_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualWanId".into(),
                    value: virtual_wan_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpnSiteResult {
            address_cidrs: o.get_field("addressCidrs"),
            device_model: o.get_field("deviceModel"),
            device_vendor: o.get_field("deviceVendor"),
            links: o.get_field("links"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            o365_policy: o.get_field("o365Policy"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            virtual_wan_id: o.get_field("virtualWanId"),
        }
    }
}
