/// Manages a Network Connection Monitor.
///
/// > **NOTE:** Any Network Connection Monitor resource created with API versions 2019-06-01 or earlier (v1) are now incompatible with this provider, which now only supports v2.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-Watcher-resources")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-Workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleExtension = extension::create(
///         "exampleExtension",
///         ExtensionArgs::builder()
///             .auto_upgrade_minor_version(true)
///             .name("example-VMExtension")
///             .publisher("Microsoft.Azure.NetworkWatcher")
///             .type_("NetworkWatcherAgentLinux")
///             .type_handler_version("1.4")
///             .virtual_machine_id("${exampleVirtualMachine.id}")
///             .build_struct(),
///     );
///     let exampleNetworkConnectionMonitor = network_connection_monitor::create(
///         "exampleNetworkConnectionMonitor",
///         NetworkConnectionMonitorArgs::builder()
///             .endpoints(
///                 vec![
///                     NetworkConnectionMonitorEndpoint::builder()
///                     .filter(NetworkConnectionMonitorEndpointFilter::builder()
///                     .items(vec![NetworkConnectionMonitorEndpointFilterItem::builder()
///                     .address("${exampleVirtualMachine.id}"). type ("AgentAddress")
///                     .build_struct(),]). type ("Include").build_struct()).name("source")
///                     .targetResourceId("${exampleVirtualMachine.id}").build_struct(),
///                     NetworkConnectionMonitorEndpoint::builder().address("mycompany.io")
///                     .name("destination").build_struct(),
///                 ],
///             )
///             .location("${exampleNetworkWatcher.location}")
///             .name("example-Monitor")
///             .network_watcher_id("${exampleNetworkWatcher.id}")
///             .notes("examplenote")
///             .output_workspace_resource_ids(vec!["${exampleAnalyticsWorkspace.id}",])
///             .test_configurations(
///                 vec![
///                     NetworkConnectionMonitorTestConfiguration::builder().name("tcpName")
///                     .protocol("Tcp")
///                     .tcpConfiguration(NetworkConnectionMonitorTestConfigurationTcpConfiguration::builder()
///                     .port(80).build_struct()).testFrequencyInSeconds(60).build_struct(),
///                 ],
///             )
///             .test_groups(
///                 vec![
///                     NetworkConnectionMonitorTestGroup::builder()
///                     .destinationEndpoints(vec!["destination",]).name("exampletg")
///                     .sourceEndpoints(vec!["source",])
///                     .testConfigurationNames(vec!["tcpName",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("testconfiguration1")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-Nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkWatcher = network_watcher::create(
///         "exampleNetworkWatcher",
///         NetworkWatcherArgs::builder()
///             .location("${example.location}")
///             .name("example-Watcher")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("example-Subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualMachine = virtual_machine::create(
///         "exampleVirtualMachine",
///         VirtualMachineArgs::builder()
///             .location("${example.location}")
///             .name("example-VM")
///             .network_interface_ids(vec!["${exampleNetworkInterface.id}",])
///             .os_profile(
///                 VirtualMachineOsProfile::builder()
///                     .adminPassword("Password1234!")
///                     .adminUsername("testadmin")
///                     .computerName("hostnametest01")
///                     .build_struct(),
///             )
///             .os_profile_linux_config(
///                 VirtualMachineOsProfileLinuxConfig::builder()
///                     .disablePasswordAuthentication(false)
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .storage_image_reference(
///                 VirtualMachineStorageImageReference::builder()
///                     .offer("0001-com-ubuntu-server-jammy")
///                     .publisher("Canonical")
///                     .sku("22_04-lts")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .storage_os_disk(
///                 VirtualMachineStorageOsDisk::builder()
///                     .caching("ReadWrite")
///                     .createOption("FromImage")
///                     .managedDiskType("Standard_LRS")
///                     .name("osdisk-example01")
///                     .build_struct(),
///             )
///             .vm_size("Standard_D2s_v3")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-Vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Network Connection Monitors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/networkConnectionMonitor:NetworkConnectionMonitor example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/networkWatchers/watcher1/connectionMonitors/connectionMonitor1
/// ```
///
pub mod network_connection_monitor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkConnectionMonitorArgs {
        /// A `endpoint` block as defined below.
        #[builder(into)]
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkConnectionMonitorEndpoint>,
        >,
        /// The Azure Region where the Network Connection Monitor should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Network Connection Monitor. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Network Watcher. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_watcher_id: pulumi_wasm_rust::Output<String>,
        /// The description of the Network Connection Monitor.
        #[builder(into, default)]
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IDs of the Log Analytics Workspace which will accept the output from the Network Connection Monitor.
        #[builder(into, default)]
        pub output_workspace_resource_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A mapping of tags which should be assigned to the Network Connection Monitor.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `test_configuration` block as defined below.
        #[builder(into)]
        pub test_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkConnectionMonitorTestConfiguration>,
        >,
        /// A `test_group` block as defined below.
        #[builder(into)]
        pub test_groups: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkConnectionMonitorTestGroup>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkConnectionMonitorResult {
        /// A `endpoint` block as defined below.
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkConnectionMonitorEndpoint>,
        >,
        /// The Azure Region where the Network Connection Monitor should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Network Connection Monitor. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Network Watcher. Changing this forces a new resource to be created.
        pub network_watcher_id: pulumi_wasm_rust::Output<String>,
        /// The description of the Network Connection Monitor.
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of IDs of the Log Analytics Workspace which will accept the output from the Network Connection Monitor.
        pub output_workspace_resource_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A mapping of tags which should be assigned to the Network Connection Monitor.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `test_configuration` block as defined below.
        pub test_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkConnectionMonitorTestConfiguration>,
        >,
        /// A `test_group` block as defined below.
        pub test_groups: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::NetworkConnectionMonitorTestGroup>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: NetworkConnectionMonitorArgs,
    ) -> NetworkConnectionMonitorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoints_binding = args.endpoints.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_watcher_id_binding = args.network_watcher_id.get_inner();
        let notes_binding = args.notes.get_inner();
        let output_workspace_resource_ids_binding = args
            .output_workspace_resource_ids
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let test_configurations_binding = args.test_configurations.get_inner();
        let test_groups_binding = args.test_groups.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/networkConnectionMonitor:NetworkConnectionMonitor"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpoints".into(),
                    value: &endpoints_binding,
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
                    name: "networkWatcherId".into(),
                    value: &network_watcher_id_binding,
                },
                register_interface::ObjectField {
                    name: "notes".into(),
                    value: &notes_binding,
                },
                register_interface::ObjectField {
                    name: "outputWorkspaceResourceIds".into(),
                    value: &output_workspace_resource_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "testConfigurations".into(),
                    value: &test_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "testGroups".into(),
                    value: &test_groups_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkWatcherId".into(),
                },
                register_interface::ResultField {
                    name: "notes".into(),
                },
                register_interface::ResultField {
                    name: "outputWorkspaceResourceIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "testConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "testGroups".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkConnectionMonitorResult {
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_watcher_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkWatcherId").unwrap(),
            ),
            notes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notes").unwrap(),
            ),
            output_workspace_resource_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputWorkspaceResourceIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            test_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testConfigurations").unwrap(),
            ),
            test_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testGroups").unwrap(),
            ),
        }
    }
}