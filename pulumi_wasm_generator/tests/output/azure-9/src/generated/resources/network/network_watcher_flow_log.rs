/// Manages a Network Watcher Flow Log.
///
/// > **Note** The `azure.network.NetworkWatcherFlowLog` creates a new storage lifecyle management rule that overwrites existing rules. Please make sure to use a `storage_account` with no existing management rules, until the issue is fixed.
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
///   test:
///     type: azure:network:NetworkSecurityGroup
///     properties:
///       name: acctestnsg
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   testNetworkWatcher:
///     type: azure:network:NetworkWatcher
///     name: test
///     properties:
///       name: acctestnw
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   testAccount:
///     type: azure:storage:Account
///     name: test
///     properties:
///       name: acctestsa
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountKind: StorageV2
///       accountReplicationType: LRS
///       enableHttpsTrafficOnly: true
///   testAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: test
///     properties:
///       name: acctestlaw
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///   testNetworkWatcherFlowLog:
///     type: azure:network:NetworkWatcherFlowLog
///     name: test
///     properties:
///       networkWatcherName: ${testNetworkWatcher.name}
///       resourceGroupName: ${example.name}
///       name: example-log
///       targetResourceId: ${test.id}
///       storageAccountId: ${testAccount.id}
///       enabled: true
///       retentionPolicy:
///         enabled: true
///         days: 7
///       trafficAnalytics:
///         enabled: true
///         workspaceId: ${testAnalyticsWorkspace.workspaceId}
///         workspaceRegion: ${testAnalyticsWorkspace.location}
///         workspaceResourceId: ${testAnalyticsWorkspace.id}
///         intervalInMinutes: 10
/// ```
///
/// ## Import
///
/// Network Watcher Flow Logs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkWatcherFlowLog:NetworkWatcherFlowLog watcher1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkWatchers/watcher1/flowLogs/log1
/// ```
///
pub mod network_watcher_flow_log {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkWatcherFlowLogArgs {
        /// Should Network Flow Logging be Enabled?
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The location where the Network Watcher Flow Log resides. Changing this forces a new resource to be created. Defaults to the `location` of the Network Watcher.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Network Watcher Flow Log. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub network_security_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Network Watcher. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_watcher_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Network Watcher was deployed. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `retention_policy` block as documented below.
        #[builder(into)]
        pub retention_policy: pulumi_wasm_rust::Output<
            super::super::types::network::NetworkWatcherFlowLogRetentionPolicy,
        >,
        /// The ID of the Storage Account where flow logs are stored.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Network Watcher Flow Log.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Resource for which to enable flow logs for. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub target_resource_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A `traffic_analytics` block as documented below.
        #[builder(into, default)]
        pub traffic_analytics: pulumi_wasm_rust::Output<
            Option<super::super::types::network::NetworkWatcherFlowLogTrafficAnalytics>,
        >,
        /// The version (revision) of the flow log. Possible values are `1` and `2`. Defaults to `1`.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct NetworkWatcherFlowLogResult {
        /// Should Network Flow Logging be Enabled?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The location where the Network Watcher Flow Log resides. Changing this forces a new resource to be created. Defaults to the `location` of the Network Watcher.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Network Watcher Flow Log. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_security_group_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Network Watcher. Changing this forces a new resource to be created.
        pub network_watcher_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Network Watcher was deployed. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `retention_policy` block as documented below.
        pub retention_policy: pulumi_wasm_rust::Output<
            super::super::types::network::NetworkWatcherFlowLogRetentionPolicy,
        >,
        /// The ID of the Storage Account where flow logs are stored.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Network Watcher Flow Log.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the Resource for which to enable flow logs for. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
        /// A `traffic_analytics` block as documented below.
        pub traffic_analytics: pulumi_wasm_rust::Output<
            Option<super::super::types::network::NetworkWatcherFlowLogTrafficAnalytics>,
        >,
        /// The version (revision) of the flow log. Possible values are `1` and `2`. Defaults to `1`.
        pub version: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkWatcherFlowLogArgs,
    ) -> NetworkWatcherFlowLogResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_security_group_id_binding = args
            .network_security_group_id
            .get_inner();
        let network_watcher_name_binding = args.network_watcher_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let retention_policy_binding = args.retention_policy.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let traffic_analytics_binding = args.traffic_analytics.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkWatcherFlowLog:NetworkWatcherFlowLog".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
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
                    name: "networkSecurityGroupId".into(),
                    value: &network_security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkWatcherName".into(),
                    value: &network_watcher_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPolicy".into(),
                    value: &retention_policy_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "trafficAnalytics".into(),
                    value: &traffic_analytics_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkSecurityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "networkWatcherName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retentionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetResourceId".into(),
                },
                register_interface::ResultField {
                    name: "trafficAnalytics".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkWatcherFlowLogResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkSecurityGroupId").unwrap(),
            ),
            network_watcher_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkWatcherName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retention_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPolicy").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
            traffic_analytics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficAnalytics").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
