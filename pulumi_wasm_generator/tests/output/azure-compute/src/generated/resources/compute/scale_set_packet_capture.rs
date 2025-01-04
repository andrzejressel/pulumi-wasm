/// Configures Network Packet Capturing against a Virtual Machine Scale Set using a Network Watcher.
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
///     let exampleLinuxVirtualMachineScaleSet = linux_virtual_machine_scale_set::create(
///         "exampleLinuxVirtualMachineScaleSet",
///         LinuxVirtualMachineScaleSetArgs::builder()
///             .admin_password("P@ssword1234!")
///             .admin_username("adminuser")
///             .computer_name_prefix("my-linux-computer-name-prefix")
///             .disable_password_authentication(false)
///             .instances(4)
///             .location("${example.location}")
///             .name("example-vmss")
///             .network_interfaces(
///                 vec![
///                     LinuxVirtualMachineScaleSetNetworkInterface::builder()
///                     .ipConfigurations(vec![LinuxVirtualMachineScaleSetNetworkInterfaceIpConfiguration::builder()
///                     .name("internal").primary(true).subnetId("${exampleSubnet.id}")
///                     .build_struct(),]).name("example").primary(true).build_struct(),
///                 ],
///             )
///             .os_disk(
///                 LinuxVirtualMachineScaleSetOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .sku("Standard_F2")
///             .source_image_reference(
///                 LinuxVirtualMachineScaleSetSourceImageReference::builder()
///                     .offer("0001-com-ubuntu-server-jammy")
///                     .publisher("Canonical")
///                     .sku("22_04-lts")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .upgrade_mode("Automatic")
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
///     let exampleScaleSetPacketCapture = scale_set_packet_capture::create(
///         "exampleScaleSetPacketCapture",
///         ScaleSetPacketCaptureArgs::builder()
///             .machine_scope(
///                 ScaleSetPacketCaptureMachineScope::builder()
///                     .excludeInstanceIds(vec!["1",])
///                     .includeInstanceIds(vec!["0",])
///                     .build_struct(),
///             )
///             .name("example-pc")
///             .network_watcher_id("${exampleNetworkWatcher.id}")
///             .storage_location(
///                 ScaleSetPacketCaptureStorageLocation::builder()
///                     .filePath("/var/captures/packet.cap")
///                     .build_struct(),
///             )
///             .virtual_machine_scale_set_id("${exampleLinuxVirtualMachineScaleSet.id}")
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
///     let exampleVirtualMachineScaleSetExtension = virtual_machine_scale_set_extension::create(
///         "exampleVirtualMachineScaleSetExtension",
///         VirtualMachineScaleSetExtensionArgs::builder()
///             .auto_upgrade_minor_version(true)
///             .automatic_upgrade_enabled(true)
///             .name("network-watcher")
///             .publisher("Microsoft.Azure.NetworkWatcher")
///             .type_("NetworkWatcherAgentLinux")
///             .type_handler_version("1.4")
///             .virtual_machine_scale_set_id("${exampleLinuxVirtualMachineScaleSet.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-vn")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **NOTE:** This Resource requires that [the Network Watcher Extension](https://docs.microsoft.com/azure/network-watcher/network-watcher-packet-capture-manage-portal#before-you-begin) is installed on the Virtual Machine Scale Set before capturing can be enabled which can be installed via the `azure.compute.VirtualMachineScaleSetExtension` resource.
///
/// ## Import
///
/// Virtual Machine Scale Set Packet Captures can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/scaleSetPacketCapture:ScaleSetPacketCapture capture1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/networkWatchers/watcher1/packetCaptures/capture1
/// ```
///
pub mod scale_set_packet_capture {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScaleSetPacketCaptureArgs {
        /// One or more `filter` blocks as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::ScaleSetPacketCaptureFilter>>,
        >,
        /// A `machine_scope` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub machine_scope: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ScaleSetPacketCaptureMachineScope>,
        >,
        /// The number of bytes captured per packet. The remaining bytes are truncated. Defaults to `0` (Entire Packet Captured). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub maximum_bytes_per_packet: pulumi_wasm_rust::Output<Option<i32>>,
        /// Maximum size of the capture in Bytes. Defaults to `1073741824` (1GB). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub maximum_bytes_per_session: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum duration of the capture session in seconds. Defaults to `18000` (5 hours). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub maximum_capture_duration_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name to use for this Network Packet Capture. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the Network Watcher. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_watcher_id: pulumi_wasm_rust::Output<String>,
        /// A `storage_location` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_location: pulumi_wasm_rust::Output<
            super::super::types::compute::ScaleSetPacketCaptureStorageLocation,
        >,
        /// The resource ID of the Virtual Machine Scale Set to capture packets from. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_scale_set_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ScaleSetPacketCaptureResult {
        /// One or more `filter` blocks as defined below. Changing this forces a new resource to be created.
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::ScaleSetPacketCaptureFilter>>,
        >,
        /// A `machine_scope` block as defined below. Changing this forces a new resource to be created.
        pub machine_scope: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::ScaleSetPacketCaptureMachineScope>,
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
            super::super::types::compute::ScaleSetPacketCaptureStorageLocation,
        >,
        /// The resource ID of the Virtual Machine Scale Set to capture packets from. Changing this forces a new resource to be created.
        pub virtual_machine_scale_set_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ScaleSetPacketCaptureArgs,
    ) -> ScaleSetPacketCaptureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let machine_scope_binding = args.machine_scope.get_inner();
        let maximum_bytes_per_packet_binding = args.maximum_bytes_per_packet.get_inner();
        let maximum_bytes_per_session_binding = args
            .maximum_bytes_per_session
            .get_inner();
        let maximum_capture_duration_in_seconds_binding = args
            .maximum_capture_duration_in_seconds
            .get_inner();
        let name_binding = args.name.get_inner();
        let network_watcher_id_binding = args.network_watcher_id.get_inner();
        let storage_location_binding = args.storage_location.get_inner();
        let virtual_machine_scale_set_id_binding = args
            .virtual_machine_scale_set_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/scaleSetPacketCapture:ScaleSetPacketCapture".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "machineScope".into(),
                    value: &machine_scope_binding,
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
                    name: "virtualMachineScaleSetId".into(),
                    value: &virtual_machine_scale_set_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "machineScope".into(),
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
                    name: "virtualMachineScaleSetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScaleSetPacketCaptureResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            machine_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineScope").unwrap(),
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
            virtual_machine_scale_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMachineScaleSetId").unwrap(),
            ),
        }
    }
}
