/// Manages a NetApp Snapshot Policy.
///
/// ## NetApp Snapshot Policy Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("East US")
///             .name("resource-group-01")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("netappaccount-01")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSnapshotPolicy = snapshot_policy::create(
///         "exampleSnapshotPolicy",
///         SnapshotPolicyArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .daily_schedule(
///                 SnapshotPolicyDailySchedule::builder()
///                     .hour(20)
///                     .minute(15)
///                     .snapshotsToKeep(2)
///                     .build_struct(),
///             )
///             .enabled(true)
///             .hourly_schedule(
///                 SnapshotPolicyHourlySchedule::builder()
///                     .minute(15)
///                     .snapshotsToKeep(4)
///                     .build_struct(),
///             )
///             .location("${example.location}")
///             .monthly_schedule(
///                 SnapshotPolicyMonthlySchedule::builder()
///                     .daysOfMonths(vec![1, 15, 20, 30,])
///                     .hour(5)
///                     .minute(45)
///                     .snapshotsToKeep(1)
///                     .build_struct(),
///             )
///             .name("snapshotpolicy-01")
///             .resource_group_name("${example.name}")
///             .weekly_schedule(
///                 SnapshotPolicyWeeklySchedule::builder()
///                     .daysOfWeeks(vec!["Monday", "Friday",])
///                     .hour(23)
///                     .minute(0)
///                     .snapshotsToKeep(1)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// NetApp Snapshot Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/snapshotPolicy:SnapshotPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.NetApp/netAppAccounts/account1/snapshotPolicies/snapshotpolicy1
/// ```
///
pub mod snapshot_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotPolicyArgs {
        /// The name of the NetApp Account in which the NetApp Snapshot Policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Sets a daily snapshot schedule. A `daily_schedule` block as defined below.
        #[builder(into, default)]
        pub daily_schedule: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::SnapshotPolicyDailySchedule>,
        >,
        /// Defines that the NetApp Snapshot Policy is enabled or not.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// Sets an hourly snapshot schedule. A `hourly_schedule` block as defined below.
        #[builder(into, default)]
        pub hourly_schedule: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::SnapshotPolicyHourlySchedule>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Sets a monthly snapshot schedule. A `monthly_schedule` block as defined below.
        #[builder(into, default)]
        pub monthly_schedule: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::SnapshotPolicyMonthlySchedule>,
        >,
        /// The name of the NetApp Snapshot Policy. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the NetApp Snapshot Policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Sets a weekly snapshot schedule. A `weekly_schedule` block as defined below.
        #[builder(into, default)]
        pub weekly_schedule: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::netapp::SnapshotPolicyWeeklySchedule>,
        >,
    }
    #[allow(dead_code)]
    pub struct SnapshotPolicyResult {
        /// The name of the NetApp Account in which the NetApp Snapshot Policy should be created. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Sets a daily snapshot schedule. A `daily_schedule` block as defined below.
        pub daily_schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::SnapshotPolicyDailySchedule>,
        >,
        /// Defines that the NetApp Snapshot Policy is enabled or not.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Sets an hourly snapshot schedule. A `hourly_schedule` block as defined below.
        pub hourly_schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::SnapshotPolicyHourlySchedule>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Sets a monthly snapshot schedule. A `monthly_schedule` block as defined below.
        pub monthly_schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::SnapshotPolicyMonthlySchedule>,
        >,
        /// The name of the NetApp Snapshot Policy. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the NetApp Snapshot Policy should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Sets a weekly snapshot schedule. A `weekly_schedule` block as defined below.
        pub weekly_schedule: pulumi_wasm_rust::Output<
            Option<super::super::types::netapp::SnapshotPolicyWeeklySchedule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SnapshotPolicyArgs,
    ) -> SnapshotPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let daily_schedule_binding = args.daily_schedule.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let hourly_schedule_binding = args
            .hourly_schedule
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let monthly_schedule_binding = args
            .monthly_schedule
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let weekly_schedule_binding = args
            .weekly_schedule
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/snapshotPolicy:SnapshotPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "dailySchedule".into(),
                    value: &daily_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "hourlySchedule".into(),
                    value: &hourly_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "monthlySchedule".into(),
                    value: &monthly_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "weeklySchedule".into(),
                    value: &weekly_schedule_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "dailySchedule".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "hourlySchedule".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "monthlySchedule".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "weeklySchedule".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SnapshotPolicyResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            daily_schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailySchedule").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            hourly_schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hourlySchedule").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            monthly_schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlySchedule").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            weekly_schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklySchedule").unwrap(),
            ),
        }
    }
}
