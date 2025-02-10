/// Manages a Express Route Port.
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
///             .location("West US")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleExpressRoutePort = express_route_port::create(
///         "exampleExpressRoutePort",
///         ExpressRoutePortArgs::builder()
///             .bandwidth_in_gbps(10)
///             .encapsulation("Dot1Q")
///             .location("${example.location}")
///             .name("port1")
///             .peering_location("Airtel-Chennai-CLS")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Express Route Ports can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRoutePort:ExpressRoutePort example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/expressRoutePorts/port1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod express_route_port {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRoutePortArgs {
        /// Bandwidth of the Express Route Port in Gbps. Changing this forces a new Express Route Port to be created.
        #[builder(into)]
        pub bandwidth_in_gbps: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The billing type of the Express Route Port. Possible values are `MeteredData` and `UnlimitedData`. Defaults to `MeteredData`.
        #[builder(into, default)]
        pub billing_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The encapsulation method used for the Express Route Port. Changing this forces a new Express Route Port to be created. Possible values are: `Dot1Q`, `QinQ`.
        #[builder(into)]
        pub encapsulation: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRoutePortIdentity>,
        >,
        /// A list of `link` blocks as defined below.
        #[builder(into, default)]
        pub link1: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRoutePortLink1>,
        >,
        /// A list of `link` blocks as defined below.
        #[builder(into, default)]
        pub link2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::ExpressRoutePortLink2>,
        >,
        /// The Azure Region where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Express Route Port. Changing this forces a new Express Route Port to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the peering location that this Express Route Port is physically mapped to. Changing this forces a new Express Route Port to be created.
        #[builder(into)]
        pub peering_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Express Route Port.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExpressRoutePortResult {
        /// Bandwidth of the Express Route Port in Gbps. Changing this forces a new Express Route Port to be created.
        pub bandwidth_in_gbps: pulumi_gestalt_rust::Output<i32>,
        /// The billing type of the Express Route Port. Possible values are `MeteredData` and `UnlimitedData`. Defaults to `MeteredData`.
        pub billing_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The encapsulation method used for the Express Route Port. Changing this forces a new Express Route Port to be created. Possible values are: `Dot1Q`, `QinQ`.
        pub encapsulation: pulumi_gestalt_rust::Output<String>,
        /// The EtherType of the Express Route Port.
        pub ethertype: pulumi_gestalt_rust::Output<String>,
        /// The resource GUID of the Express Route Port.
        pub guid: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::network::ExpressRoutePortIdentity>,
        >,
        /// A list of `link` blocks as defined below.
        pub link1: pulumi_gestalt_rust::Output<
            super::super::types::network::ExpressRoutePortLink1,
        >,
        /// A list of `link` blocks as defined below.
        pub link2: pulumi_gestalt_rust::Output<
            super::super::types::network::ExpressRoutePortLink2,
        >,
        /// The Azure Region where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum transmission unit of the Express Route Port.
        pub mtu: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Express Route Port. Changing this forces a new Express Route Port to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the peering location that this Express Route Port is physically mapped to. Changing this forces a new Express Route Port to be created.
        pub peering_location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Express Route Port should exist. Changing this forces a new Express Route Port to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Express Route Port.
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
        args: ExpressRoutePortArgs,
    ) -> ExpressRoutePortResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bandwidth_in_gbps_binding = args.bandwidth_in_gbps.get_output(context);
        let billing_type_binding = args.billing_type.get_output(context);
        let encapsulation_binding = args.encapsulation.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let link1_binding = args.link1.get_output(context);
        let link2_binding = args.link2.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let peering_location_binding = args.peering_location.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/expressRoutePort:ExpressRoutePort".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bandwidthInGbps".into(),
                    value: bandwidth_in_gbps_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingType".into(),
                    value: billing_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encapsulation".into(),
                    value: encapsulation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "link1".into(),
                    value: link1_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "link2".into(),
                    value: link2_binding.get_id(),
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
                    name: "peeringLocation".into(),
                    value: peering_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExpressRoutePortResult {
            bandwidth_in_gbps: o.get_field("bandwidthInGbps"),
            billing_type: o.get_field("billingType"),
            encapsulation: o.get_field("encapsulation"),
            ethertype: o.get_field("ethertype"),
            guid: o.get_field("guid"),
            identity: o.get_field("identity"),
            link1: o.get_field("link1"),
            link2: o.get_field("link2"),
            location: o.get_field("location"),
            mtu: o.get_field("mtu"),
            name: o.get_field("name"),
            peering_location: o.get_field("peeringLocation"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
