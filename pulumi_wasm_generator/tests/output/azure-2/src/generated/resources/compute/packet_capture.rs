/// Configures Network Packet Capturing against a Virtual Machine using a Network Watcher.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleExtension = extension::create(
///         "exampleExtension",
///         ExtensionArgs::builder()
///             .auto_upgrade_minor_version(true)
///             .name("network-watcher")
///             .publisher("Microsoft.Azure.NetworkWatcher")
///             .type_("NetworkWatcherAgentLinux")
///             .type_handler_version("1.4")
///             .virtual_machine_id("${exampleVirtualMachine.id}")
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
///             .name("example-nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkWatcher = network_watcher::create(
///         "exampleNetworkWatcher",
///         NetworkWatcherArgs::builder()
///             .location("${example.location}")
///             .name("example-nw")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePacketCapture = packet_capture::create(
///         "examplePacketCapture",
///         PacketCaptureArgs::builder()
///             .name("example-pc")
///             .network_watcher_id("${exampleNetworkWatcher.id}")
///             .storage_location(
///                 PacketCaptureStorageLocation::builder()
///                     .storageAccountId("${exampleAccount.id}")
///                     .build_struct(),
///             )
///             .virtual_machine_id("${exampleVirtualMachine.id}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualMachine = virtual_machine::create(
///         "exampleVirtualMachine",
///         VirtualMachineArgs::builder()
///             .location("${example.location}")
///             .name("example-vm")
///             .network_interface_ids(vec!["${exampleNetworkInterface.id}",])
///             .os_profile(
///                 VirtualMachineOsProfile::builder()
///                     .adminPassword("Password1234!")
///                     .adminUsername("testadmin")
///                     .computerName("pctest-vm")
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
///                     .name("osdisk")
///                     .build_struct(),
///             )
///             .vm_size("Standard_F2")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **NOTE:** This Resource requires that [the Network Watcher Virtual Machine Extension](https://docs.microsoft.com/azure/network-watcher/network-watcher-packet-capture-manage-portal#before-you-begin) is installed on the Virtual Machine before capturing can be enabled which can be installed via the `azure.compute.Extension` resource.
///
/// ## Import
///
/// Virtual Machine Packet Captures can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/packetCapture:PacketCapture capture1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkWatchers/watcher1/packetCaptures/capture1
/// ```
///
pub mod packet_capture {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PacketCaptureArgs {
        /// One or more `filter` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::PacketCaptureFilter>>,
        >,
        /// The number of bytes captured per packet. The remaining bytes are truncated. Defaults to `0` (Entire Packet Captured). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub maximum_bytes_per_packet: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Maximum size of the capture in Bytes. Defaults to `1073741824` (1GB). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub maximum_bytes_per_session: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The maximum duration of the capture session in seconds. Defaults to `18000` (5 hours). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub maximum_capture_duration_in_seconds: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name to use for this Network Packet Capture. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Network Watcher. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_watcher_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `storage_location` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_location: pulumi_wasm_rust::InputOrOutput<
            super::super::types::compute::PacketCaptureStorageLocation,
        >,
        /// The resource ID of the target Virtual Machine to capture packets from. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PacketCaptureResult {
        /// One or more `filter` blocks as defined below. Changing this forces a new resource to be created.
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::PacketCaptureFilter>>,
        >,
        /// The number of bytes captured per packet. The remaining bytes are truncated. Defaults to `0` (Entire Packet Captured). Changing this forces a new resource to be created.
        pub maximum_bytes_per_packet: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum size of the capture in Bytes. Defaults to `1073741824` (1GB). Changing this forces a new resource to be created.
        pub maximum_bytes_per_session: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum duration of the capture session in seconds. Defaults to `18000` (5 hours). Changing this forces a new resource to be created.
        pub maximum_capture_duration_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name to use for this Network Packet Capture. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Network Watcher. Changing this forces a new resource to be created.
        pub network_watcher_id: pulumi_wasm_rust::Output<String>,
        /// A `storage_location` block as defined below. Changing this forces a new resource to be created.
        pub storage_location: pulumi_wasm_rust::Output<
            super::super::types::compute::PacketCaptureStorageLocation,
        >,
        /// The resource ID of the target Virtual Machine to capture packets from. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PacketCaptureArgs,
    ) -> PacketCaptureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let maximum_bytes_per_packet_binding = args
            .maximum_bytes_per_packet
            .get_output(context)
            .get_inner();
        let maximum_bytes_per_session_binding = args
            .maximum_bytes_per_session
            .get_output(context)
            .get_inner();
        let maximum_capture_duration_in_seconds_binding = args
            .maximum_capture_duration_in_seconds
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_watcher_id_binding = args
            .network_watcher_id
            .get_output(context)
            .get_inner();
        let storage_location_binding = args
            .storage_location
            .get_output(context)
            .get_inner();
        let virtual_machine_id_binding = args
            .virtual_machine_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/packetCapture:PacketCapture".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "maximumBytesPerPacket".into(),
                    value: &maximum_bytes_per_packet_binding,
                },
                register_interface::ObjectField {
                    name: "maximumBytesPerSession".into(),
                    value: &maximum_bytes_per_session_binding,
                },
                register_interface::ObjectField {
                    name: "maximumCaptureDurationInSeconds".into(),
                    value: &maximum_capture_duration_in_seconds_binding,
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
                    name: "storageLocation".into(),
                    value: &storage_location_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "maximumBytesPerPacket".into(),
                },
                register_interface::ResultField {
                    name: "maximumBytesPerSession".into(),
                },
                register_interface::ResultField {
                    name: "maximumCaptureDurationInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkWatcherId".into(),
                },
                register_interface::ResultField {
                    name: "storageLocation".into(),
                },
                register_interface::ResultField {
                    name: "virtualMachineId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PacketCaptureResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            maximum_bytes_per_packet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumBytesPerPacket").unwrap(),
            ),
            maximum_bytes_per_session: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumBytesPerSession").unwrap(),
            ),
            maximum_capture_duration_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumCaptureDurationInSeconds").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_watcher_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkWatcherId").unwrap(),
            ),
            storage_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageLocation").unwrap(),
            ),
            virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineId").unwrap(),
            ),
        }
    }
}
