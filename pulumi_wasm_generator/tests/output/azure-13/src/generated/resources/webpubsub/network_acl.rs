/// Manages the Network ACL for a Web Pubsub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: terraform-webpubsub
///       location: east us
///   exampleService:
///     type: azure:webpubsub:Service
///     name: example
///     properties:
///       name: tfex-webpubsub
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard_S1
///       capacity: 1
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
///           - webpubsub
///   exampleNetworkAcl:
///     type: azure:webpubsub:NetworkAcl
///     name: example
///     properties:
///       webPubsubId: ${exampleService.id}
///       defaultAction: Allow
///       publicNetwork:
///         deniedRequestTypes:
///           - ClientConnection
///       privateEndpoints:
///         - id: ${exampleEndpoint.id}
///           deniedRequestTypes:
///             - RESTAPI
///             - ClientConnection
///     options:
///       dependsOn:
///         - ${exampleEndpoint}
/// ```
///
/// ## Import
///
/// Network ACLs for a Web Pubsub service can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:webpubsub/networkAcl:NetworkAcl example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SignalRService/webPubSub/webpubsub1
/// ```
///
pub mod network_acl {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkAclArgs {
        /// The default action to control the network access when no other rule matches. Possible values are `Allow` and `Deny`. Defaults to `Deny`.
        #[builder(into, default)]
        pub default_action: pulumi_wasm_rust::Output<Option<String>>,
        /// A `private_endpoint` block as defined below.
        #[builder(into, default)]
        pub private_endpoints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::webpubsub::NetworkAclPrivateEndpoint>>,
        >,
        /// A `public_network` block as defined below.
        #[builder(into)]
        pub public_network: pulumi_wasm_rust::Output<
            super::super::types::webpubsub::NetworkAclPublicNetwork,
        >,
        /// The ID of the Web Pubsub service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub web_pubsub_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkAclResult {
        /// The default action to control the network access when no other rule matches. Possible values are `Allow` and `Deny`. Defaults to `Deny`.
        pub default_action: pulumi_wasm_rust::Output<Option<String>>,
        /// A `private_endpoint` block as defined below.
        pub private_endpoints: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::webpubsub::NetworkAclPrivateEndpoint>>,
        >,
        /// A `public_network` block as defined below.
        pub public_network: pulumi_wasm_rust::Output<
            super::super::types::webpubsub::NetworkAclPublicNetwork,
        >,
        /// The ID of the Web Pubsub service. Changing this forces a new resource to be created.
        pub web_pubsub_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkAclArgs) -> NetworkAclResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_action_binding = args.default_action.get_inner();
        let private_endpoints_binding = args.private_endpoints.get_inner();
        let public_network_binding = args.public_network.get_inner();
        let web_pubsub_id_binding = args.web_pubsub_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:webpubsub/networkAcl:NetworkAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultAction".into(),
                    value: &default_action_binding,
                },
                register_interface::ObjectField {
                    name: "privateEndpoints".into(),
                    value: &private_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetwork".into(),
                    value: &public_network_binding,
                },
                register_interface::ObjectField {
                    name: "webPubsubId".into(),
                    value: &web_pubsub_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultAction".into(),
                },
                register_interface::ResultField {
                    name: "privateEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "publicNetwork".into(),
                },
                register_interface::ResultField {
                    name: "webPubsubId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkAclResult {
            default_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAction").unwrap(),
            ),
            private_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateEndpoints").unwrap(),
            ),
            public_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetwork").unwrap(),
            ),
            web_pubsub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webPubsubId").unwrap(),
            ),
        }
    }
}
