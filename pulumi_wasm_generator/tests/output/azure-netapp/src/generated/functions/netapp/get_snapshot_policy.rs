pub mod get_snapshot_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotPolicyArgs {
        /// The name of the NetApp account where the NetApp Snapshot Policy exists.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the NetApp Snapshot Policy.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Name of the Resource Group where the NetApp Snapshot Policy exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotPolicyResult {
        /// The name of the NetApp Account in which the NetApp Snapshot Policy was created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Daily snapshot schedule.
        pub daily_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyDailySchedule>,
        >,
        /// Defines that the NetApp Snapshot Policy is enabled or not.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Hourly snapshot schedule.
        pub hourly_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyHourlySchedule>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// List of the days of the month when the snapshots will be created.
        pub monthly_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyMonthlySchedule>,
        >,
        /// The name of the NetApp Snapshot Policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group where the NetApp Snapshot Policy should be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Weekly snapshot schedule.
        pub weekly_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyWeeklySchedule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSnapshotPolicyArgs) -> GetSnapshotPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:netapp/getSnapshotPolicy:getSnapshotPolicy".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "dailySchedules".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "hourlySchedules".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "monthlySchedules".into(),
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
                    name: "weeklySchedules".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSnapshotPolicyResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            daily_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailySchedules").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            hourly_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hourlySchedules").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            monthly_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlySchedules").unwrap(),
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
            weekly_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("weeklySchedules").unwrap(),
            ),
        }
    }
}
