/// Manages an SAP Discovery Virtual Instance.
///
/// > **Note:** Before using this resource, it's required to submit the request of registering the Resource Provider with Azure CLI `az provider register --namespace "Microsoft.Workloads"`. The Resource Provider can take a while to register, you can check the status by running `az provider show --namespace "Microsoft.Workloads" --query "registrationState"`. Once this outputs "Registered" the Resource Provider is available for use.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-sapvis
///       location: West Europe
///   exampleDiscoveryVirtualInstance:
///     type: azure:workloadssap:DiscoveryVirtualInstance
///     name: example
///     properties:
///       name: X01
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       environment: NonProd
///       sapProduct: S4HANA
///       centralServerVirtualMachineId: /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/exampleRG/providers/Microsoft.Compute/virtualMachines/csvm1
///       managedStorageAccountName: managedsa
///       identity:
///         type: UserAssigned
///         identityIds:
///           - /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/exampleRG/providers/Microsoft.ManagedIdentity/userAssignedIdentities/uai1
/// ```
///
/// ## Import
///
/// SAP Discovery Virtual Instances can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:workloadssap/discoveryVirtualInstance:DiscoveryVirtualInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Workloads/sapVirtualInstances/vis1
/// ```
///
pub mod discovery_virtual_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiscoveryVirtualInstanceArgs {
        /// The ID of the Virtual Machine of the Central Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub central_server_virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// The environment type for the SAP Discovery Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub environment: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::workloadssap::DiscoveryVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the managed Resource Group for the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the custom Storage Account created by the service in the managed Resource Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub managed_storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SAP Product type for the SAP Discovery Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sap_product: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SAP Discovery Virtual Instance.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DiscoveryVirtualInstanceResult {
        /// The ID of the Virtual Machine of the Central Server. Changing this forces a new resource to be created.
        pub central_server_virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// The environment type for the SAP Discovery Virtual Instance. Possible values are `NonProd` and `Prod`. Changing this forces a new resource to be created.
        pub environment: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::workloadssap::DiscoveryVirtualInstanceIdentity>,
        >,
        /// The Azure Region where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the managed Resource Group for the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        pub managed_resource_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the custom Storage Account created by the service in the managed Resource Group. Changing this forces a new resource to be created.
        pub managed_storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the SAP Discovery Virtual Instance. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the SAP Discovery Virtual Instance should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SAP Product type for the SAP Discovery Virtual Instance. Possible values are `ECC`, `Other` and `S4HANA`. Changing this forces a new resource to be created.
        pub sap_product: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SAP Discovery Virtual Instance.
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
        args: DiscoveryVirtualInstanceArgs,
    ) -> DiscoveryVirtualInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let central_server_virtual_machine_id_binding = args
            .central_server_virtual_machine_id
            .get_inner();
        let environment_binding = args.environment.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_inner();
        let managed_storage_account_name_binding = args
            .managed_storage_account_name
            .get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sap_product_binding = args.sap_product.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:workloadssap/discoveryVirtualInstance:DiscoveryVirtualInstance"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "centralServerVirtualMachineId".into(),
                    value: &central_server_virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedResourceGroupName".into(),
                    value: &managed_resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "managedStorageAccountName".into(),
                    value: &managed_storage_account_name_binding,
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
                    name: "sapProduct".into(),
                    value: &sap_product_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "centralServerVirtualMachineId".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedResourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "managedStorageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sapProduct".into(),
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
        DiscoveryVirtualInstanceResult {
            central_server_virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("centralServerVirtualMachineId").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedResourceGroupName").unwrap(),
            ),
            managed_storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedStorageAccountName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sap_product: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sapProduct").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
