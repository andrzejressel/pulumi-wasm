/// Manages automated shutdown schedules for Azure VMs that are not within an Azure DevTest Lab. While this is part of the DevTest Labs service in Azure,
/// this resource applies only to standard VMs, not DevTest Lab VMs. To manage automated shutdown schedules for DevTest Lab VMs, reference the
/// `azure.devtest.Schedule` resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: sample-rg
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: sample-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: sample-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: sample-nic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipConfigurations:
///         - name: testconfiguration1
///           subnetId: ${exampleSubnet.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: SampleVM
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       size: Standard_B2s
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       osDisk:
///         name: myosdisk-example
///         caching: ReadWrite
///         storageAccountType: Standard_LRS
///       adminUsername: testadmin
///       adminPassword: Password1234!
///       disablePasswordAuthentication: false
///   exampleGlobalVMShutdownSchedule:
///     type: azure:devtest:GlobalVMShutdownSchedule
///     name: example
///     properties:
///       virtualMachineId: ${exampleLinuxVirtualMachine.id}
///       location: ${example.location}
///       enabled: true
///       dailyRecurrenceTime: '1100'
///       timezone: Pacific Standard Time
///       notificationSettings:
///         enabled: true
///         timeInMinutes: '60'
///         webhookUrl: https://sample-webhook-url.example.com
/// ```
///
/// ## Import
///
/// An existing Dev Test Global Shutdown Schedule can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/globalVMShutdownSchedule:GlobalVMShutdownSchedule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/sample-rg/providers/Microsoft.DevTestLab/schedules/shutdown-computevm-SampleVM
/// ```
///
/// The name of the resource within the `resource id` will always follow the format `shutdown-computevm-<VM Name>` where `<VM Name>` is replaced by the name of the target Virtual Machine
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod global_vm_shutdown_schedule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalVMShutdownScheduleArgs {
        /// The time each day when the schedule takes effect. Must match the format HHmm where HH is 00-23 and mm is 00-59 (e.g. 0930, 2300, etc.)
        #[builder(into)]
        pub daily_recurrence_time: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable the schedule. Possible values are `true` and `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The location where the schedule is created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The notification setting of a schedule. A `notification_settings` block as defined below.
        #[builder(into)]
        pub notification_settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::devtest::GlobalVmShutdownScheduleNotificationSettings,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time zone ID (e.g. Pacific Standard time). Refer to this guide for a [full list of accepted time zone names](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
        #[builder(into)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID of the target ARM-based Virtual Machine. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GlobalVMShutdownScheduleResult {
        /// The time each day when the schedule takes effect. Must match the format HHmm where HH is 00-23 and mm is 00-59 (e.g. 0930, 2300, etc.)
        pub daily_recurrence_time: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable the schedule. Possible values are `true` and `false`. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The location where the schedule is created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The notification setting of a schedule. A `notification_settings` block as defined below.
        pub notification_settings: pulumi_gestalt_rust::Output<
            super::super::types::devtest::GlobalVmShutdownScheduleNotificationSettings,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time zone ID (e.g. Pacific Standard time). Refer to this guide for a [full list of accepted time zone names](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
        pub timezone: pulumi_gestalt_rust::Output<String>,
        /// The resource ID of the target ARM-based Virtual Machine. Changing this forces a new resource to be created.
        pub virtual_machine_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalVMShutdownScheduleArgs,
    ) -> GlobalVMShutdownScheduleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let daily_recurrence_time_binding = args
            .daily_recurrence_time
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let notification_settings_binding = args
            .notification_settings
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timezone_binding = args.timezone.get_output(context);
        let virtual_machine_id_binding = args.virtual_machine_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devtest/globalVMShutdownSchedule:GlobalVMShutdownSchedule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dailyRecurrenceTime".into(),
                    value: &daily_recurrence_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationSettings".into(),
                    value: &notification_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachineId".into(),
                    value: &virtual_machine_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GlobalVMShutdownScheduleResult {
            daily_recurrence_time: o.get_field("dailyRecurrenceTime"),
            enabled: o.get_field("enabled"),
            location: o.get_field("location"),
            notification_settings: o.get_field("notificationSettings"),
            tags: o.get_field("tags"),
            timezone: o.get_field("timezone"),
            virtual_machine_id: o.get_field("virtualMachineId"),
        }
    }
}
