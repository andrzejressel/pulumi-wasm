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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_disk_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataDiskAttachmentArgs {
        /// Specifies the caching requirements for this Data Disk. Possible values include `None`, `ReadOnly` and `ReadWrite`.
        #[builder(into)]
        pub caching: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Create Option of the Data Disk, such as `Empty` or `Attach`. Defaults to `Attach`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub create_option: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Logical Unit Number of the Data Disk, which needs to be unique within the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lun: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of an existing Managed Disk which should be attached. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_disk_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Virtual Machine to which the Data Disk should be attached. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies if Write Accelerator is enabled on the disk. This can only be enabled on `Premium_LRS` managed disks with no caching and [M-Series VMs](https://docs.microsoft.com/azure/virtual-machines/workloads/sap/how-to-enable-write-accelerator). Defaults to `false`.
        #[builder(into, default)]
        pub write_accelerator_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct DataDiskAttachmentResult {
        /// Specifies the caching requirements for this Data Disk. Possible values include `None`, `ReadOnly` and `ReadWrite`.
        pub caching: pulumi_gestalt_rust::Output<String>,
        /// The Create Option of the Data Disk, such as `Empty` or `Attach`. Defaults to `Attach`. Changing this forces a new resource to be created.
        pub create_option: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Logical Unit Number of the Data Disk, which needs to be unique within the Virtual Machine. Changing this forces a new resource to be created.
        pub lun: pulumi_gestalt_rust::Output<i32>,
        /// The ID of an existing Managed Disk which should be attached. Changing this forces a new resource to be created.
        pub managed_disk_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Virtual Machine to which the Data Disk should be attached. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies if Write Accelerator is enabled on the disk. This can only be enabled on `Premium_LRS` managed disks with no caching and [M-Series VMs](https://docs.microsoft.com/azure/virtual-machines/workloads/sap/how-to-enable-write-accelerator). Defaults to `false`.
        pub write_accelerator_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataDiskAttachmentArgs,
    ) -> DataDiskAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let caching_binding = args.caching.get_output(context);
        let create_option_binding = args.create_option.get_output(context);
        let lun_binding = args.lun.get_output(context);
        let managed_disk_id_binding = args.managed_disk_id.get_output(context);
        let virtual_machine_id_binding = args.virtual_machine_id.get_output(context);
        let write_accelerator_enabled_binding = args
            .write_accelerator_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/dataDiskAttachment:DataDiskAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "caching".into(),
                    value: &caching_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createOption".into(),
                    value: &create_option_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lun".into(),
                    value: &lun_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedDiskId".into(),
                    value: &managed_disk_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "writeAcceleratorEnabled".into(),
                    value: &write_accelerator_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataDiskAttachmentResult {
            caching: o.get_field("caching"),
            create_option: o.get_field("createOption"),
            lun: o.get_field("lun"),
            managed_disk_id: o.get_field("managedDiskId"),
            virtual_machine_id: o.get_field("virtualMachineId"),
            write_accelerator_enabled: o.get_field("writeAcceleratorEnabled"),
        }
    }
}
