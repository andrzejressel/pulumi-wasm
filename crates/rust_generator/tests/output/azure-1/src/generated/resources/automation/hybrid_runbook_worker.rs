/// Manages a Automation Hybrid Runbook Worker.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hybrid_runbook_worker {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HybridRunbookWorkerArgs {
        /// The name of the automation account in which the Hybrid Worker is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the virtual machine used for this HybridWorker. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub vm_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the HybridWorker Group. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub worker_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specify the ID of this HybridWorker in UUID notation. Changing this forces a new Automation to be created.
        #[builder(into)]
        pub worker_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HybridRunbookWorkerResult {
        /// The name of the automation account in which the Hybrid Worker is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The IP address of assigned machine.
        pub ip: pulumi_gestalt_rust::Output<String>,
        /// Last Heartbeat from the Worker.
        pub last_seen_date_time: pulumi_gestalt_rust::Output<String>,
        /// The registration time of the worker machine.
        pub registration_date_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Automation should exist. Changing this forces a new Automation to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the virtual machine used for this HybridWorker. Changing this forces a new Automation to be created.
        pub vm_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the HybridWorker Group. Changing this forces a new Automation to be created.
        pub worker_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specify the ID of this HybridWorker in UUID notation. Changing this forces a new Automation to be created.
        pub worker_id: pulumi_gestalt_rust::Output<String>,
        /// The name of HybridWorker.
        pub worker_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the HybridWorker, the possible values are `HybridV1` and `HybridV2`.
        pub worker_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridRunbookWorkerArgs,
    ) -> HybridRunbookWorkerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let vm_resource_id_binding = args.vm_resource_id.get_output(context);
        let worker_group_name_binding = args.worker_group_name.get_output(context);
        let worker_id_binding = args.worker_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/hybridRunbookWorker:HybridRunbookWorker".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmResourceId".into(),
                    value: &vm_resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workerGroupName".into(),
                    value: &worker_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workerId".into(),
                    value: &worker_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        HybridRunbookWorkerResult {
            automation_account_name: o.get_field("automationAccountName"),
            ip: o.get_field("ip"),
            last_seen_date_time: o.get_field("lastSeenDateTime"),
            registration_date_time: o.get_field("registrationDateTime"),
            resource_group_name: o.get_field("resourceGroupName"),
            vm_resource_id: o.get_field("vmResourceId"),
            worker_group_name: o.get_field("workerGroupName"),
            worker_id: o.get_field("workerId"),
            worker_name: o.get_field("workerName"),
            worker_type: o.get_field("workerType"),
        }
    }
}
