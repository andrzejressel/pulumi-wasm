/// Manages a Virtual Machine Restore Point.
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
///   exampleRestorePoint:
///     type: azure:compute:RestorePoint
///     name: example
///     properties:
///       name: example-restore-point
///       restorePointCollectionId: ${test.id}
/// ```
///
/// ## Import
///
/// Virtual Machine Restore Point can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/restorePoint:RestorePoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/restorePointCollections/collection1/restorePoints/restorePoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod restore_point {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestorePointArgs {
        /// Is Crash Consistent the Consistency Mode of the Virtual Machine Restore Point. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub crash_consistency_mode_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A list of disks that will be excluded from the Virtual Machine Restore Point. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub excluded_disks: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the name of the Virtual Machine Restore Point. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub virtual_machine_restore_point_collection_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct RestorePointResult {
        /// Is Crash Consistent the Consistency Mode of the Virtual Machine Restore Point. Defaults to `false`. Changing this forces a new resource to be created.
        pub crash_consistency_mode_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of disks that will be excluded from the Virtual Machine Restore Point. Changing this forces a new resource to be created.
        pub excluded_disks: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the name of the Virtual Machine Restore Point. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub virtual_machine_restore_point_collection_id: pulumi_gestalt_rust::Output<
            String,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RestorePointArgs,
    ) -> RestorePointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let crash_consistency_mode_enabled_binding = args
            .crash_consistency_mode_enabled
            .get_output(context);
        let excluded_disks_binding = args.excluded_disks.get_output(context);
        let name_binding = args.name.get_output(context);
        let virtual_machine_restore_point_collection_id_binding = args
            .virtual_machine_restore_point_collection_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:compute/restorePoint:RestorePoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "crashConsistencyModeEnabled".into(),
                    value: &crash_consistency_mode_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludedDisks".into(),
                    value: &excluded_disks_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineRestorePointCollectionId".into(),
                    value: &virtual_machine_restore_point_collection_id_binding
                        .drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RestorePointResult {
            crash_consistency_mode_enabled: o.get_field("crashConsistencyModeEnabled"),
            excluded_disks: o.get_field("excludedDisks"),
            name: o.get_field("name"),
            virtual_machine_restore_point_collection_id: o
                .get_field("virtualMachineRestorePointCollectionId"),
        }
    }
}
