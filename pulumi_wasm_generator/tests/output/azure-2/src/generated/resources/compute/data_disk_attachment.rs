/// Manages attaching a Disk to a Virtual Machine.
///
/// > **NOTE:** Data Disks can be attached either directly on the `azure.compute.VirtualMachine` resource, or using the `azure.compute.DataDiskAttachment` resource - but the two cannot be used together. If both are used against the same Virtual Machine, spurious changes will occur.
///
/// > **Please Note:** only Managed Disks are supported via this separate resource, Unmanaged Disks can be attached using the `storage_data_disk` block in the `azure.compute.VirtualMachine` resource.
///
/// ## Example Usage
///
/// ```yaml
/// configuration:
///   prefix:
///     type: string
///     default: example
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: ${prefix}-resources
///       location: West Europe
///   main:
///     type: azure:network:VirtualNetwork
///     properties:
///       name: ${prefix}-network
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   internal:
///     type: azure:network:Subnet
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${main.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   mainNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: main
///     properties:
///       name: ${prefix}-nic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${internal.id}
///           privateIpAddressAllocation: Dynamic
///   exampleVirtualMachine:
///     type: azure:compute:VirtualMachine
///     name: example
///     properties:
///       name: ${vmName}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       networkInterfaceIds:
///         - ${mainNetworkInterface.id}
///       vmSize: Standard_F2
///       storageImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       storageOsDisk:
///         name: myosdisk1
///         caching: ReadWrite
///         createOption: FromImage
///         managedDiskType: Standard_LRS
///       osProfile:
///         computerName: ${vmName}
///         adminUsername: testadmin
///         adminPassword: Password1234!
///       osProfileLinuxConfig:
///         disablePasswordAuthentication: false
///   exampleManagedDisk:
///     type: azure:compute:ManagedDisk
///     name: example
///     properties:
///       name: ${vmName}-disk1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageAccountType: Standard_LRS
///       createOption: Empty
///       diskSizeGb: 10
///   exampleDataDiskAttachment:
///     type: azure:compute:DataDiskAttachment
///     name: example
///     properties:
///       managedDiskId: ${exampleManagedDisk.id}
///       virtualMachineId: ${exampleVirtualMachine.id}
///       lun: '10'
///       caching: ReadWrite
/// variables:
///   vmName: ${prefix}-vm
/// ```
///
/// ## Import
///
/// Virtual Machines Data Disk Attachments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/dataDiskAttachment:DataDiskAttachment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/machine1/dataDisks/disk1
/// ```
///
pub mod data_disk_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataDiskAttachmentArgs {
        /// Specifies the caching requirements for this Data Disk. Possible values include `None`, `ReadOnly` and `ReadWrite`.
        #[builder(into)]
        pub caching: pulumi_wasm_rust::Output<String>,
        /// The Create Option of the Data Disk, such as `Empty` or `Attach`. Defaults to `Attach`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub create_option: pulumi_wasm_rust::Output<Option<String>>,
        /// The Logical Unit Number of the Data Disk, which needs to be unique within the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lun: pulumi_wasm_rust::Output<i32>,
        /// The ID of an existing Managed Disk which should be attached. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_disk_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Machine to which the Data Disk should be attached. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// Specifies if Write Accelerator is enabled on the disk. This can only be enabled on `Premium_LRS` managed disks with no caching and [M-Series VMs](https://docs.microsoft.com/azure/virtual-machines/workloads/sap/how-to-enable-write-accelerator). Defaults to `false`.
        #[builder(into, default)]
        pub write_accelerator_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct DataDiskAttachmentResult {
        /// Specifies the caching requirements for this Data Disk. Possible values include `None`, `ReadOnly` and `ReadWrite`.
        pub caching: pulumi_wasm_rust::Output<String>,
        /// The Create Option of the Data Disk, such as `Empty` or `Attach`. Defaults to `Attach`. Changing this forces a new resource to be created.
        pub create_option: pulumi_wasm_rust::Output<Option<String>>,
        /// The Logical Unit Number of the Data Disk, which needs to be unique within the Virtual Machine. Changing this forces a new resource to be created.
        pub lun: pulumi_wasm_rust::Output<i32>,
        /// The ID of an existing Managed Disk which should be attached. Changing this forces a new resource to be created.
        pub managed_disk_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Virtual Machine to which the Data Disk should be attached. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// Specifies if Write Accelerator is enabled on the disk. This can only be enabled on `Premium_LRS` managed disks with no caching and [M-Series VMs](https://docs.microsoft.com/azure/virtual-machines/workloads/sap/how-to-enable-write-accelerator). Defaults to `false`.
        pub write_accelerator_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataDiskAttachmentArgs) -> DataDiskAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let caching_binding = args.caching.get_inner();
        let create_option_binding = args.create_option.get_inner();
        let lun_binding = args.lun.get_inner();
        let managed_disk_id_binding = args.managed_disk_id.get_inner();
        let virtual_machine_id_binding = args.virtual_machine_id.get_inner();
        let write_accelerator_enabled_binding = args
            .write_accelerator_enabled
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/dataDiskAttachment:DataDiskAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "caching".into(),
                    value: &caching_binding,
                },
                register_interface::ObjectField {
                    name: "createOption".into(),
                    value: &create_option_binding,
                },
                register_interface::ObjectField {
                    name: "lun".into(),
                    value: &lun_binding,
                },
                register_interface::ObjectField {
                    name: "managedDiskId".into(),
                    value: &managed_disk_id_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "writeAcceleratorEnabled".into(),
                    value: &write_accelerator_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "caching".into(),
                },
                register_interface::ResultField {
                    name: "createOption".into(),
                },
                register_interface::ResultField {
                    name: "lun".into(),
                },
                register_interface::ResultField {
                    name: "managedDiskId".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineId".into(),
                },
                register_interface::ResultField {
                    name: "writeAcceleratorEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataDiskAttachmentResult {
            caching: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caching").unwrap(),
            ),
            create_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createOption").unwrap(),
            ),
            lun: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lun").unwrap(),
            ),
            managed_disk_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedDiskId").unwrap(),
            ),
            virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineId").unwrap(),
            ),
            write_accelerator_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writeAcceleratorEnabled").unwrap(),
            ),
        }
    }
}
