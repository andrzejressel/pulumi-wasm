/// Manages a Connection for a Virtual Hub.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.1.0/24")
///             .location("${example.location}")
///             .name("example-hub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHubConnection = virtual_hub_connection::create(
///         "exampleVirtualHubConnection",
///         VirtualHubConnectionArgs::builder()
///             .name("example-vhub")
///             .remote_virtual_network_id("${exampleVirtualNetwork.id}")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["172.16.0.0/12",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
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
/// }
/// ```
///
/// ## Import
///
/// Virtual Hub Connection's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualHubConnection:VirtualHubConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/hub1/hubVirtualNetworkConnections/connection1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_hub_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubConnectionArgs {
        /// Should Internet Security be enabled to secure internet traffic? Defaults to `false`.
        #[builder(into, default)]
        pub internet_security_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Name which should be used for this Connection, which must be unique within the Virtual Hub. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Network which the Virtual Hub should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub remote_virtual_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `routing` block as defined below.
        #[builder(into, default)]
        pub routing: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::VirtualHubConnectionRouting>,
        >,
        /// The ID of the Virtual Hub within which this connection should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubConnectionResult {
        /// Should Internet Security be enabled to secure internet traffic? Defaults to `false`.
        pub internet_security_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Name which should be used for this Connection, which must be unique within the Virtual Hub. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Network which the Virtual Hub should be connected to. Changing this forces a new resource to be created.
        pub remote_virtual_network_id: pulumi_gestalt_rust::Output<String>,
        /// A `routing` block as defined below.
        pub routing: pulumi_gestalt_rust::Output<
            super::super::types::network::VirtualHubConnectionRouting,
        >,
        /// The ID of the Virtual Hub within which this connection should be created. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualHubConnectionArgs,
    ) -> VirtualHubConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let internet_security_enabled_binding = args
            .internet_security_enabled
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let remote_virtual_network_id_binding = args
            .remote_virtual_network_id
            .get_output(context);
        let routing_binding = args.routing.get_output(context);
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/virtualHubConnection:VirtualHubConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetSecurityEnabled".into(),
                    value: internet_security_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteVirtualNetworkId".into(),
                    value: remote_virtual_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routing".into(),
                    value: routing_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubId".into(),
                    value: virtual_hub_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualHubConnectionResult {
            internet_security_enabled: o.get_field("internetSecurityEnabled"),
            name: o.get_field("name"),
            remote_virtual_network_id: o.get_field("remoteVirtualNetworkId"),
            routing: o.get_field("routing"),
            virtual_hub_id: o.get_field("virtualHubId"),
        }
    }
}
