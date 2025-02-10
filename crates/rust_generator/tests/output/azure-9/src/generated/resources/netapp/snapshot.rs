/// Manages a NetApp Snapshot.
///
/// ## NetApp Snapshot Usage
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
///       name: example-virtualnetwork
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
///       delegations:
///         - name: netapp
///           serviceDelegation:
///             name: Microsoft.Netapp/volumes
///             actions:
///               - Microsoft.Network/networkinterfaces/*
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   exampleAccount:
///     type: azure:netapp:Account
///     name: example
///     properties:
///       name: example-netappaccount
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   examplePool:
///     type: azure:netapp:Pool
///     name: example
///     properties:
///       name: example-netapppool
///       accountName: ${exampleAccount.name}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       serviceLevel: Premium
///       sizeInTb: '4'
///   exampleVolume:
///     type: azure:netapp:Volume
///     name: example
///     properties:
///       name: example-netappvolume
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountName: ${exampleAccount.name}
///       poolName: ${examplePool.name}
///       volumePath: my-unique-file-path
///       serviceLevel: Premium
///       subnetId: ${exampleSubnet.id}
///       storageQuotaInGb: '100'
///   exampleSnapshot:
///     type: azure:netapp:Snapshot
///     name: example
///     properties:
///       name: example-netappsnapshot
///       accountName: ${exampleAccount.name}
///       poolName: ${examplePool.name}
///       volumeName: ${exampleVolume.name}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// NetApp Snapshot can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/snapshot:Snapshot example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.NetApp/netAppAccounts/account1/capacityPools/pool1/volumes/volume1/snapshots/snapshot1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the NetApp Snapshot. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the NetApp pool in which the NetApp Volume should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group where the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the NetApp volume in which the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub volume_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the NetApp Snapshot. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the NetApp pool in which the NetApp Volume should be created. Changing this forces a new resource to be created.
        pub pool_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the NetApp volume in which the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        pub volume_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let pool_name_binding = args.pool_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let volume_name_binding = args.volume_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:netapp/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "poolName".into(),
                    value: pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeName".into(),
                    value: volume_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotResult {
            account_name: o.get_field("accountName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            pool_name: o.get_field("poolName"),
            resource_group_name: o.get_field("resourceGroupName"),
            volume_name: o.get_field("volumeName"),
        }
    }
}
