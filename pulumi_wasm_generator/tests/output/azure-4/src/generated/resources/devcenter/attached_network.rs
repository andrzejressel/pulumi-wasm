/// Manages a Dev Center Attached Network.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-dcan
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleDevCenter:
///     type: azure:devcenter:DevCenter
///     name: example
///     properties:
///       name: example-dc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
///   exampleNetworkConnection:
///     type: azure:devcenter:NetworkConnection
///     name: example
///     properties:
///       name: example-dcnc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       domainJoinType: AzureADJoin
///       subnetId: ${exampleSubnet.id}
///   exampleAttachedNetwork:
///     type: azure:devcenter:AttachedNetwork
///     name: example
///     properties:
///       name: example-dcet
///       devCenterId: ${exampleDevCenter.id}
///       networkConnectionId: ${exampleNetworkConnection.id}
/// ```
///
/// ## Import
///
/// An existing Dev Center Attached Network can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/attachedNetwork:AttachedNetwork example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevCenter/devCenters/dc1/attachedNetworks/et1
/// ```
///
pub mod attached_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttachedNetworkArgs {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Dev Center Attached Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Dev Center Network Connection you want to attach. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_connection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AttachedNetworkResult {
        /// The ID of the associated Dev Center. Changing this forces a new resource to be created.
        pub dev_center_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Dev Center Attached Network. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Dev Center Network Connection you want to attach. Changing this forces a new resource to be created.
        pub network_connection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AttachedNetworkArgs) -> AttachedNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dev_center_id_binding = args.dev_center_id.get_inner();
        let name_binding = args.name.get_inner();
        let network_connection_id_binding = args.network_connection_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/attachedNetwork:AttachedNetwork".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConnectionId".into(),
                    value: &network_connection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "devCenterId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AttachedNetworkResult {
            dev_center_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devCenterId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConnectionId").unwrap(),
            ),
        }
    }
}
