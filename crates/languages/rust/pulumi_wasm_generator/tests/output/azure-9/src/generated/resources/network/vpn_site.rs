/// Manages a VPN Site.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod vpn_site {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnSiteArgs {
        /// Specifies a list of IP address CIDRs that are located on your on-premises site. Traffic destined for these address spaces is routed to your local site.
        ///
        /// > **NOTE:** The `address_cidrs` has to be set when the `link.bgp` isn't specified.
        #[builder(into, default)]
        pub address_cidrs: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The model of the VPN device.
        #[builder(into, default)]
        pub device_model: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the VPN device vendor.
        #[builder(into, default)]
        pub device_vendor: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `link` blocks as defined below.
        #[builder(into, default)]
        pub links: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::network::VpnSiteLink>>,
        >,
        /// The Azure Region where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this VPN Site. Changing this forces a new VPN Site to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An `o365_policy` block as defined below.
        #[builder(into, default)]
        pub o365_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::network::VpnSiteO365Policy>,
        >,
        /// The name of the Resource Group where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the VPN Site.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Wan where this VPN site resides in. Changing this forces a new VPN Site to be created.
        #[builder(into)]
        pub virtual_wan_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnSiteResult {
        /// Specifies a list of IP address CIDRs that are located on your on-premises site. Traffic destined for these address spaces is routed to your local site.
        ///
        /// > **NOTE:** The `address_cidrs` has to be set when the `link.bgp` isn't specified.
        pub address_cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The model of the VPN device.
        pub device_model: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the VPN device vendor.
        pub device_vendor: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `link` blocks as defined below.
        pub links: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::network::VpnSiteLink>>,
        >,
        /// The Azure Region where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this VPN Site. Changing this forces a new VPN Site to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An `o365_policy` block as defined below.
        pub o365_policy: pulumi_wasm_rust::Output<
            super::super::types::network::VpnSiteO365Policy,
        >,
        /// The name of the Resource Group where the VPN Site should exist. Changing this forces a new VPN Site to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the VPN Site.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Virtual Wan where this VPN site resides in. Changing this forces a new VPN Site to be created.
        pub virtual_wan_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpnSiteArgs,
    ) -> VpnSiteResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_cidrs_binding = args.address_cidrs.get_output(context).get_inner();
        let device_model_binding = args.device_model.get_output(context).get_inner();
        let device_vendor_binding = args.device_vendor.get_output(context).get_inner();
        let links_binding = args.links.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let o365_policy_binding = args.o365_policy.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let virtual_wan_id_binding = args.virtual_wan_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/vpnSite:VpnSite".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressCidrs".into(),
                    value: &address_cidrs_binding,
                },
                register_interface::ObjectField {
                    name: "deviceModel".into(),
                    value: &device_model_binding,
                },
                register_interface::ObjectField {
                    name: "deviceVendor".into(),
                    value: &device_vendor_binding,
                },
                register_interface::ObjectField {
                    name: "links".into(),
                    value: &links_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "o365Policy".into(),
                    value: &o365_policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualWanId".into(),
                    value: &virtual_wan_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpnSiteResult {
            address_cidrs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addressCidrs"),
            ),
            device_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deviceModel"),
            ),
            device_vendor: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deviceVendor"),
            ),
            links: pulumi_wasm_rust::__private::into_domain(o.extract_field("links")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            o365_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("o365Policy"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            virtual_wan_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualWanId"),
            ),
        }
    }
}
