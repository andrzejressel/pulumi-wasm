/// Manages a local network gateway connection over which specific connections can be configured.
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
///             .name("localNetworkGWTest")
///             .build_struct(),
///     );
///     let home = local_network_gateway::create(
///         "home",
///         LocalNetworkGatewayArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .gateway_address("12.13.14.15")
///             .location("${example.location}")
///             .name("backHome")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Local Network Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/localNetworkGateway:LocalNetworkGateway lng1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/localNetworkGateways/lng1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod local_network_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalNetworkGatewayArgs {
        /// The list of string CIDRs representing the address spaces the gateway exposes.
        #[builder(into, default)]
        pub address_spaces: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `bgp_settings` block as defined below containing the Local Network Gateway's BGP speaker settings.
        #[builder(into, default)]
        pub bgp_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::LocalNetworkGatewayBgpSettings>,
        >,
        /// The gateway IP address to connect with.
        #[builder(into, default)]
        pub gateway_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The gateway FQDN to connect with.
        ///
        /// > **NOTE:** Either `gateway_address` or `gateway_fqdn` should be specified.
        #[builder(into, default)]
        pub gateway_fqdn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location/region where the local network gateway is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the local network gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the local network gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocalNetworkGatewayResult {
        /// The list of string CIDRs representing the address spaces the gateway exposes.
        pub address_spaces: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `bgp_settings` block as defined below containing the Local Network Gateway's BGP speaker settings.
        pub bgp_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::LocalNetworkGatewayBgpSettings>,
        >,
        /// The gateway IP address to connect with.
        pub gateway_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The gateway FQDN to connect with.
        ///
        /// > **NOTE:** Either `gateway_address` or `gateway_fqdn` should be specified.
        pub gateway_fqdn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location/region where the local network gateway is created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the local network gateway. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the local network gateway. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocalNetworkGatewayArgs,
    ) -> LocalNetworkGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_spaces_binding = args.address_spaces.get_output(context);
        let bgp_settings_binding = args.bgp_settings.get_output(context);
        let gateway_address_binding = args.gateway_address.get_output(context);
        let gateway_fqdn_binding = args.gateway_fqdn.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/localNetworkGateway:LocalNetworkGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressSpaces".into(),
                    value: &address_spaces_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpSettings".into(),
                    value: &bgp_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayAddress".into(),
                    value: &gateway_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gatewayFqdn".into(),
                    value: &gateway_fqdn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocalNetworkGatewayResult {
            address_spaces: o.get_field("addressSpaces"),
            bgp_settings: o.get_field("bgpSettings"),
            gateway_address: o.get_field("gatewayAddress"),
            gateway_fqdn: o.get_field("gatewayFqdn"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
