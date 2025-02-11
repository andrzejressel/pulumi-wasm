/// Manages an implicit Data Disk of a Virtual Machine.
///
/// > **Note:** The Implicit Data Disk will be deleted instantly after this resource is destroyed. If you want to detach this disk only, you may set `detach_implicit_data_disk_on_deletion` field to `true` within the `virtual_machine` block in the provider `features` block.
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
///   exampleSnapshot:
///     type: azure:compute:Snapshot
///     name: example
///     properties:
///       name: ${vmName}-snapshot1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       createOption: Copy
///       sourceUri: ${exampleManagedDisk.id}
///   exampleImplicitDataDiskFromSource:
///     type: azure:compute:ImplicitDataDiskFromSource
///     name: example
///     properties:
///       name: ${vmName}-implicitdisk1
///       virtualMachineId: ${testAzurermVirtualMachine.id}
///       lun: '0'
///       caching: None
///       createOption: Copy
///       diskSizeGb: 20
///       sourceResourceId: ${test.id}
/// variables:
///   vmName: ${prefix}-vm
/// ```
///
/// ## Import
///
/// The implicit Data Disk of the Virtual Machine can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/implicitDataDiskFromSource:ImplicitDataDiskFromSource example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/virtualMachines/machine1/dataDisks/disk1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod implicit_data_disk_from_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImplicitDataDiskFromSourceArgs {
        /// Specifies the caching requirements for this Data Disk. Possible values are `ReadOnly` and `ReadWrite`.
        #[builder(into, default)]
        pub caching: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Create Option of the Data Disk. The only possible value is `Copy`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub create_option: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the size of the Data Disk in gigabytes. Changing this forces a new resource to be created.
        #[builder(into)]
        pub disk_size_gb: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The Logical Unit Number of the Data Disk, which needs to be unique within the Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lun: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the name of this Data Disk. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the source resource which this Data Disk was created from. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Virtual Machine to which the Data Disk should be attached. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies if Write Accelerator is enabled on the disk. This can only be enabled on `Premium_LRS` managed disks with no caching and [M-Series VMs](https://docs.microsoft.com/azure/virtual-machines/workloads/sap/how-to-enable-write-accelerator). Defaults to `false`.
        #[builder(into, default)]
        pub write_accelerator_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ImplicitDataDiskFromSourceResult {
        /// Specifies the caching requirements for this Data Disk. Possible values are `ReadOnly` and `ReadWrite`.
        pub caching: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Create Option of the Data Disk. The only possible value is `Copy`. Changing this forces a new resource to be created.
        pub create_option: pulumi_gestalt_rust::Output<String>,
        /// Specifies the size of the Data Disk in gigabytes. Changing this forces a new resource to be created.
        pub disk_size_gb: pulumi_gestalt_rust::Output<i32>,
        /// The Logical Unit Number of the Data Disk, which needs to be unique within the Virtual Machine. Changing this forces a new resource to be created.
        pub lun: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the name of this Data Disk. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the source resource which this Data Disk was created from. Changing this forces a new resource to be created.
        pub source_resource_id: pulumi_gestalt_rust::Output<String>,
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
        args: ImplicitDataDiskFromSourceArgs,
    ) -> ImplicitDataDiskFromSourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let caching_binding = args.caching.get_output(context);
        let create_option_binding = args.create_option.get_output(context);
        let disk_size_gb_binding = args.disk_size_gb.get_output(context);
        let lun_binding = args.lun.get_output(context);
        let name_binding = args.name.get_output(context);
        let source_resource_id_binding = args.source_resource_id.get_output(context);
        let virtual_machine_id_binding = args.virtual_machine_id.get_output(context);
        let write_accelerator_enabled_binding = args
            .write_accelerator_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/implicitDataDiskFromSource:ImplicitDataDiskFromSource"
                .into(),
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
                    name: "diskSizeGb".into(),
                    value: &disk_size_gb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lun".into(),
                    value: &lun_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceResourceId".into(),
                    value: &source_resource_id_binding.drop_type(),
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
        ImplicitDataDiskFromSourceResult {
            caching: o.get_field("caching"),
            create_option: o.get_field("createOption"),
            disk_size_gb: o.get_field("diskSizeGb"),
            lun: o.get_field("lun"),
            name: o.get_field("name"),
            source_resource_id: o.get_field("sourceResourceId"),
            virtual_machine_id: o.get_field("virtualMachineId"),
            write_accelerator_enabled: o.get_field("writeAcceleratorEnabled"),
        }
    }
}
