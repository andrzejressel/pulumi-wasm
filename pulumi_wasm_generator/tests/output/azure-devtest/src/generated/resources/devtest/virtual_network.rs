/// Manages a Virtual Network within a DevTest Lab.
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
///   exampleLab:
///     type: azure:devtest:Lab
///     name: example
///     properties:
///       name: example-devtestlab
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         Sydney: Australia
///   exampleVirtualNetwork:
///     type: azure:devtest:VirtualNetwork
///     name: example
///     properties:
///       name: example-network
///       labName: ${exampleLab.name}
///       resourceGroupName: ${example.name}
///       subnet:
///         usePublicIpAddress: Allow
///         useInVirtualMachineCreation: Allow
/// ```
///
/// ## Import
///
/// DevTest Virtual Networks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/virtualNetwork:VirtualNetwork network1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevTestLab/labs/lab1/virtualNetworks/network1
/// ```
///
pub mod virtual_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkArgs {
        /// A description for the Virtual Network.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Virtual Network should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Dev Test Virtual Network. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `subnet` block as defined below.
        #[builder(into, default)]
        pub subnet: pulumi_wasm_rust::Output<
            Option<super::super::types::devtest::VirtualNetworkSubnet>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkResult {
        /// A description for the Virtual Network.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Virtual Network should be created. Changing this forces a new resource to be created.
        pub lab_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Dev Test Virtual Network. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `subnet` block as defined below.
        pub subnet: pulumi_wasm_rust::Output<
            super::super::types::devtest::VirtualNetworkSubnet,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique immutable identifier of the Dev Test Virtual Network.
        pub unique_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualNetworkArgs) -> VirtualNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let lab_name_binding = args.lab_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subnet_binding = args.subnet.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devtest/virtualNetwork:VirtualNetwork".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labName".into(),
                    value: &lab_name_binding,
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
                    name: "subnet".into(),
                    value: &subnet_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "labName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subnet".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "uniqueIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualNetworkResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            lab_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subnet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnet").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            unique_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueIdentifier").unwrap(),
            ),
        }
    }
}