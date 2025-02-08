/// Manages an Azure Route Server
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vn
///       addressSpaces:
///         - 10.0.0.0/16
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       tags:
///         environment: Production
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: RouteServerSubnet
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       resourceGroupName: ${example.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example-pip
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       allocationMethod: Static
///       sku: Standard
///   exampleRouteServer:
///     type: azure:network:RouteServer
///     name: example
///     properties:
///       name: example-routerserver
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Standard
///       publicIpAddressId: ${examplePublicIp.id}
///       subnetId: ${exampleSubnet.id}
///       branchToBranchTrafficEnabled: true
/// ```
///
/// ## Import
///
/// Route Server can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/routeServer:RouteServer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/routeServer1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteServerArgs {
        /// Whether to enable route exchange between Azure Route Server and the gateway(s)
        #[builder(into, default)]
        pub branch_to_branch_traffic_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the Route Server should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Route Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Public IP Address. This option is required since September 1st 2021. Changing this forces a new resource to be created.
        #[builder(into)]
        pub public_ip_address_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the Route Server should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of the Route Server. The only possible value is `Standard`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet that the Route Server will reside. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Azure Route Server requires a dedicated subnet named RouteServerSubnet. The subnet size has to be at least /27 or short prefix (such as /26 or /25) and cannot be attached to any security group, otherwise, you'll receive an error message when deploying the Route Server
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RouteServerResult {
        /// Whether to enable route exchange between Azure Route Server and the gateway(s)
        pub branch_to_branch_traffic_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the Route Server should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Route Server. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Public IP Address. This option is required since September 1st 2021. Changing this forces a new resource to be created.
        pub public_ip_address_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Route Server should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub routing_state: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Route Server. The only possible value is `Standard`. Changing this forces a new resource to be created.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet that the Route Server will reside. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Azure Route Server requires a dedicated subnet named RouteServerSubnet. The subnet size has to be at least /27 or short prefix (such as /26 or /25) and cannot be attached to any security group, otherwise, you'll receive an error message when deploying the Route Server
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub virtual_router_asn: pulumi_gestalt_rust::Output<i32>,
        pub virtual_router_ips: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RouteServerArgs,
    ) -> RouteServerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let branch_to_branch_traffic_enabled_binding = args
            .branch_to_branch_traffic_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_ip_address_id_binding = args
            .public_ip_address_id
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/routeServer:RouteServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "branchToBranchTrafficEnabled".into(),
                    value: &branch_to_branch_traffic_enabled_binding,
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
                    name: "publicIpAddressId".into(),
                    value: &public_ip_address_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouteServerResult {
            branch_to_branch_traffic_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("branchToBranchTrafficEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            public_ip_address_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicIpAddressId"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            routing_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingState"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            virtual_router_asn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualRouterAsn"),
            ),
            virtual_router_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualRouterIps"),
            ),
        }
    }
}
