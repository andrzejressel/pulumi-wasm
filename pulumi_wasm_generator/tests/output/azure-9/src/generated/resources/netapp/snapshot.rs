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
pub mod snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the NetApp Snapshot. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the NetApp pool in which the NetApp Volume should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp volume in which the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub volume_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// The name of the NetApp account in which the NetApp Pool should be created. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Snapshot. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp pool in which the NetApp Volume should be created. Changing this forces a new resource to be created.
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp volume in which the NetApp Snapshot should be created. Changing this forces a new resource to be created.
        pub volume_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SnapshotArgs) -> SnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let pool_name_binding = args.pool_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let volume_name_binding = args.volume_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "poolName".into(),
                    value: &pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "volumeName".into(),
                    value: &volume_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "poolName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "volumeName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SnapshotResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pool_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            volume_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeName").unwrap(),
            ),
        }
    }
}
