/// Manages a Virtual Machine Restore Point Collection.
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
///       name: example-network
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
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: example-nic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: example-machine
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       size: Standard_F2
///       adminUsername: adminuser
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       adminSshKeys:
///         - username: adminuser
///           publicKey:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: ~/.ssh/id_rsa.pub
///               return: result
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleRestorePointCollection:
///     type: azure:compute:RestorePointCollection
///     name: example
///     properties:
///       name: example-collection
///       resourceGroupName: ${example.name}
///       location: ${exampleLinuxVirtualMachine.location}
///       sourceVirtualMachineId: ${exampleLinuxVirtualMachine.id}
/// ```
///
/// ## Import
///
/// Virtual Machine Restore Point Collections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/restorePointCollection:RestorePointCollection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/restorePointCollections/collection1
/// ```
///
pub mod restore_point_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestorePointCollectionArgs {
        /// The Azure location where the Virtual Machine Restore Point Collection should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Virtual Machine Restore Point Collection. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Virtual Machine Restore Point Collection should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual machine that will be associated with this Virtual Machine Restore Point Collection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to this Virtual Machine Restore Point Collection.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RestorePointCollectionResult {
        /// The Azure location where the Virtual Machine Restore Point Collection should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Virtual Machine Restore Point Collection. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Virtual Machine Restore Point Collection should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual machine that will be associated with this Virtual Machine Restore Point Collection. Changing this forces a new resource to be created.
        pub source_virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to this Virtual Machine Restore Point Collection.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RestorePointCollectionArgs,
    ) -> RestorePointCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let source_virtual_machine_id_binding = args
            .source_virtual_machine_id
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/restorePointCollection:RestorePointCollection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceVirtualMachineId".into(),
                    value: &source_virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sourceVirtualMachineId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RestorePointCollectionResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source_virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceVirtualMachineId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
