/// Manages an Azure Managed Lustre File System.
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
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleFileSystem:
///     type: azure:managedlustre:FileSystem
///     name: example
///     properties:
///       name: example-amlfs
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       skuName: AMLFS-Durable-Premium-250
///       subnetId: ${exampleSubnet.id}
///       storageCapacityInTb: 8
///       zones:
///         - '2'
///       maintenanceWindow:
///         dayOfWeek: Friday
///         timeOfDayUtc: 22:00
/// ```
///
/// ## Import
///
/// Azure Managed Lustre File Systems can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:managedlustre/fileSystem:FileSystem example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageCache/amlFilesystems/amlFilesystem1
/// ```
///
pub mod file_system {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FileSystemArgs {
        /// An `encryption_key` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_key` forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_key: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::managedlustre::FileSystemEncryptionKey>,
        >,
        /// A `hsm_setting` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub hsm_setting: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::managedlustre::FileSystemHsmSetting>,
        >,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::managedlustre::FileSystemIdentity>,
        >,
        /// The Azure Region where the Azure Managed Lustre File System should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `maintenance_window` block as defined below.
        #[builder(into)]
        pub maintenance_window: pulumi_wasm_rust::InputOrOutput<
            super::super::types::managedlustre::FileSystemMaintenanceWindow,
        >,
        /// The name which should be used for this Azure Managed Lustre File System. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure Managed Lustre File System should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SKU name for the Azure Managed Lustre File System. Possible values are `AMLFS-Durable-Premium-40`, `AMLFS-Durable-Premium-125`, `AMLFS-Durable-Premium-250` and `AMLFS-Durable-Premium-500`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The size of the Azure Managed Lustre File System in TiB. The valid values for this field are dependant on which `sku_name` has been defined in the configuration file. For more information on the valid values for this field please see the [product documentation](https://learn.microsoft.com/azure/azure-managed-lustre/create-file-system-resource-manager#file-system-type-and-size-options). Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_capacity_in_tb: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The resource ID of the Subnet that is used for managing the Azure Managed Lustre file system and for client-facing operations. This subnet should have at least a /24 subnet mask within the Virtual Network's address space. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure Managed Lustre File System.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of availability zones for the Azure Managed Lustre File System. Changing this forces a new resource to be created.
        #[builder(into)]
        pub zones: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct FileSystemResult {
        /// An `encryption_key` block as defined below.
        ///
        /// > **NOTE:** Removing `encryption_key` forces a new resource to be created.
        pub encryption_key: pulumi_wasm_rust::Output<
            Option<super::super::types::managedlustre::FileSystemEncryptionKey>,
        >,
        /// A `hsm_setting` block as defined below. Changing this forces a new resource to be created.
        pub hsm_setting: pulumi_wasm_rust::Output<
            Option<super::super::types::managedlustre::FileSystemHsmSetting>,
        >,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::managedlustre::FileSystemIdentity>,
        >,
        /// The Azure Region where the Azure Managed Lustre File System should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `maintenance_window` block as defined below.
        pub maintenance_window: pulumi_wasm_rust::Output<
            super::super::types::managedlustre::FileSystemMaintenanceWindow,
        >,
        /// IP Address of Managed Lustre File System Services.
        pub mgs_address: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure Managed Lustre File System. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Managed Lustre File System should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU name for the Azure Managed Lustre File System. Possible values are `AMLFS-Durable-Premium-40`, `AMLFS-Durable-Premium-125`, `AMLFS-Durable-Premium-250` and `AMLFS-Durable-Premium-500`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The size of the Azure Managed Lustre File System in TiB. The valid values for this field are dependant on which `sku_name` has been defined in the configuration file. For more information on the valid values for this field please see the [product documentation](https://learn.microsoft.com/azure/azure-managed-lustre/create-file-system-resource-manager#file-system-type-and-size-options). Changing this forces a new resource to be created.
        pub storage_capacity_in_tb: pulumi_wasm_rust::Output<i32>,
        /// The resource ID of the Subnet that is used for managing the Azure Managed Lustre file system and for client-facing operations. This subnet should have at least a /24 subnet mask within the Virtual Network's address space. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Managed Lustre File System.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of availability zones for the Azure Managed Lustre File System. Changing this forces a new resource to be created.
        pub zones: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FileSystemArgs,
    ) -> FileSystemResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryption_key_binding = args.encryption_key.get_output(context).get_inner();
        let hsm_setting_binding = args.hsm_setting.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let storage_capacity_in_tb_binding = args
            .storage_capacity_in_tb
            .get_output(context)
            .get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let zones_binding = args.zones.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:managedlustre/fileSystem:FileSystem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptionKey".into(),
                    value: &encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "hsmSetting".into(),
                    value: &hsm_setting_binding,
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
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
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
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageCapacityInTb".into(),
                    value: &storage_capacity_in_tb_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryptionKey".into(),
                },
                register_interface::ResultField {
                    name: "hsmSetting".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "mgsAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storageCapacityInTb".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FileSystemResult {
            encryption_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionKey").unwrap(),
            ),
            hsm_setting: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmSetting").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            mgs_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mgsAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storage_capacity_in_tb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageCapacityInTb").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
