/// Manages the Network ACL for a SignalR service.
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
///   exampleService:
///     type: azure:signalr:Service
///     name: example
///     properties:
///       name: example-signalr
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Standard_S1
///         capacity: 1
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       addressSpaces:
///         - 10.5.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.5.2.0/24
///       enforcePrivateLinkEndpointNetworkPolicies: true
///   exampleEndpoint:
///     type: azure:privatelink:Endpoint
///     name: example
///     properties:
///       name: example-privateendpoint
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       subnetId: ${exampleSubnet.id}
///       privateServiceConnection:
///         name: psc-sig-test
///         isManualConnection: false
///         privateConnectionResourceId: ${exampleService.id}
///         subresourceNames:
///           - signalr
///   exampleServiceNetworkAcl:
///     type: azure:signalr:ServiceNetworkAcl
///     name: example
///     properties:
///       signalrServiceId: ${exampleService.id}
///       defaultAction: Deny
///       publicNetwork:
///         allowedRequestTypes:
///           - ClientConnection
///       privateEndpoints:
///         - id: ${exampleEndpoint.id}
///           allowedRequestTypes:
///             - ServerConnection
/// ```
///
/// ## Import
///
/// Network ACLs for a SignalR service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:signalr/serviceNetworkAcl:ServiceNetworkAcl example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/signalR/signalr1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_network_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceNetworkAclArgs {
        /// The default action to control the network access when no other rule matches. Possible values are `Allow` and `Deny`.
        #[builder(into)]
        pub default_action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `private_endpoint` block as defined below.
        #[builder(into, default)]
        pub private_endpoints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::signalr::ServiceNetworkAclPrivateEndpoint>>,
        >,
        /// A `public_network` block as defined below.
        #[builder(into)]
        pub public_network: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::signalr::ServiceNetworkAclPublicNetwork,
        >,
        /// The ID of the SignalR service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub signalr_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceNetworkAclResult {
        /// The default action to control the network access when no other rule matches. Possible values are `Allow` and `Deny`.
        pub default_action: pulumi_gestalt_rust::Output<String>,
        /// A `private_endpoint` block as defined below.
        pub private_endpoints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::signalr::ServiceNetworkAclPrivateEndpoint>>,
        >,
        /// A `public_network` block as defined below.
        pub public_network: pulumi_gestalt_rust::Output<
            super::super::types::signalr::ServiceNetworkAclPublicNetwork,
        >,
        /// The ID of the SignalR service. Changing this forces a new resource to be created.
        pub signalr_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceNetworkAclArgs,
    ) -> ServiceNetworkAclResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_action_binding = args.default_action.get_output(context);
        let private_endpoints_binding = args.private_endpoints.get_output(context);
        let public_network_binding = args.public_network.get_output(context);
        let signalr_service_id_binding = args.signalr_service_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:signalr/serviceNetworkAcl:ServiceNetworkAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultAction".into(),
                    value: &default_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateEndpoints".into(),
                    value: &private_endpoints_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetwork".into(),
                    value: &public_network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signalrServiceId".into(),
                    value: &signalr_service_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceNetworkAclResult {
            default_action: o.get_field("defaultAction"),
            private_endpoints: o.get_field("privateEndpoints"),
            public_network: o.get_field("publicNetwork"),
            signalr_service_id: o.get_field("signalrServiceId"),
        }
    }
}
