/// Manages a Automation Hybrid Runbook Worker.
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
///             .location("${example.location}")
///             .name("example-account")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleHybridRunbookWorker = hybrid_runbook_worker::create(
///         "exampleHybridRunbookWorker",
///         HybridRunbookWorkerArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .resource_group_name("${example.name}")
///             .vm_resource_id("${exampleLinuxVirtualMachine.id}")
///             .worker_group_name("${exampleHybridRunbookWorkerGroup.name}")
///             .worker_id("00000000-0000-0000-0000-000000000000")
///             .build_struct(),
///     );
///     let exampleHybridRunbookWorkerGroup = hybrid_runbook_worker_group::create(
///         "exampleHybridRunbookWorkerGroup",
///         HybridRunbookWorkerGroupArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinuxVirtualMachine = linux_virtual_machine::create(
///         "exampleLinuxVirtualMachine",
///         LinuxVirtualMachineArgs::builder()
///             .admin_password("Password1234!")
///             .admin_username("testadmin")
///             .disable_password_authentication(false)
///             .location("${example.location}")
///             .name("example-vm")
///             .network_interface_ids(vec!["${exampleNetworkInterface.id}",])
///             .os_disk(
///                 LinuxVirtualMachineOsDisk::builder()
///                     .caching("ReadWrite")
///                     .storageAccountType("Standard_LRS")
///                     .build_struct(),
///             )
///             .resource_group_name("${example.name}")
///             .size("Standard_B1s")
///             .source_image_reference(
///                 LinuxVirtualMachineSourceImageReference::builder()
///                     .offer("0001-com-ubuntu-server-jammy")
///                     .publisher("Canonical")
///                     .sku("22_04-lts")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleNetworkInterface = network_interface::create(
///         "exampleNetworkInterface",
///         NetworkInterfaceArgs::builder()
///             .ip_configurations(
///                 vec![
///                     NetworkInterfaceIpConfiguration::builder().name("vm-example")
///                     .privateIpAddressAllocation("Dynamic")
///                     .subnetId("${exampleSubnet.id}").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example-nic")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["192.168.1.0/24",])
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["192.168.1.0/24",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/hybridRunbookWorker:HybridRunbookWorker example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/hybridRunbookWorkerGroups/group1/hybridRunbookWorkers/00000000-0000-0000-0000-000000000000
/// ```
///
pub mod hybrid_runbook_worker {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HybridRunbookWorkerArgs {
        /// The name of the automation account in which the Hybrid Worker is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual machine used for this HybridWorker. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub vm_resource_id: pulumi_wasm_rust::Output<String>,
        /// The name of the HybridWorker Group. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub worker_group_name: pulumi_wasm_rust::Output<String>,
        /// Specify the ID of this HybridWorker in UUID notation. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub worker_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct HybridRunbookWorkerResult {
        /// The name of the automation account in which the Hybrid Worker is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The IP address of assigned machine.
        pub ip: pulumi_wasm_rust::Output<String>,
        /// Last Heartbeat from the Worker.
        pub last_seen_date_time: pulumi_wasm_rust::Output<String>,
        /// The registration time of the worker machine.
        pub registration_date_time: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the virtual machine used for this HybridWorker. Changing this forces a new Automation to be created.
        pub vm_resource_id: pulumi_wasm_rust::Output<String>,
        /// The name of the HybridWorker Group. Changing this forces a new Automation to be created.
        pub worker_group_name: pulumi_wasm_rust::Output<String>,
        /// Specify the ID of this HybridWorker in UUID notation. Changing this forces a new Automation to be created.
        pub worker_id: pulumi_wasm_rust::Output<String>,
        /// The name of HybridWorker.
        pub worker_name: pulumi_wasm_rust::Output<String>,
        /// The type of the HybridWorker, the possible values are `HybridV1` and `HybridV2`.
        pub worker_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: HybridRunbookWorkerArgs,
    ) -> HybridRunbookWorkerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args.automation_account_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let vm_resource_id_binding = args.vm_resource_id.get_inner();
        let worker_group_name_binding = args.worker_group_name.get_inner();
        let worker_id_binding = args.worker_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/hybridRunbookWorker:HybridRunbookWorker".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "vmResourceId".into(),
                    value: &vm_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "workerGroupName".into(),
                    value: &worker_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "workerId".into(),
                    value: &worker_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "ip".into(),
                },
                register_interface::ResultField {
                    name: "lastSeenDateTime".into(),
                },
                register_interface::ResultField {
                    name: "registrationDateTime".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "vmResourceId".into(),
                },
                register_interface::ResultField {
                    name: "workerGroupName".into(),
                },
                register_interface::ResultField {
                    name: "workerId".into(),
                },
                register_interface::ResultField {
                    name: "workerName".into(),
                },
                register_interface::ResultField {
                    name: "workerType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HybridRunbookWorkerResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            ip: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ip").unwrap()),
            last_seen_date_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastSeenDateTime").unwrap(),
            ),
            registration_date_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrationDateTime").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            vm_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmResourceId").unwrap(),
            ),
            worker_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerGroupName").unwrap(),
            ),
            worker_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerId").unwrap(),
            ),
            worker_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerName").unwrap(),
            ),
            worker_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerType").unwrap(),
            ),
        }
    }
}