#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotPolicyArgs {
        /// The name of the NetApp account where the NetApp Snapshot Policy exists.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the NetApp Snapshot Policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the NetApp Snapshot Policy exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotPolicyResult {
        /// The name of the NetApp Account in which the NetApp Snapshot Policy was created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// Daily snapshot schedule.
        pub daily_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyDailySchedule>,
        >,
        /// Defines that the NetApp Snapshot Policy is enabled or not.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Hourly snapshot schedule.
        pub hourly_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyHourlySchedule>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// List of the days of the month when the snapshots will be created.
        pub monthly_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyMonthlySchedule>,
        >,
        /// The name of the NetApp Snapshot Policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the NetApp Snapshot Policy should be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Weekly snapshot schedule.
        pub weekly_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::netapp::GetSnapshotPolicyWeeklySchedule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSnapshotPolicyArgs,
    ) -> GetSnapshotPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:netapp/getSnapshotPolicy:getSnapshotPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotPolicyResult {
            account_name: o.get_field("accountName"),
            daily_schedules: o.get_field("dailySchedules"),
            enabled: o.get_field("enabled"),
            hourly_schedules: o.get_field("hourlySchedules"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            monthly_schedules: o.get_field("monthlySchedules"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            weekly_schedules: o.get_field("weeklySchedules"),
        }
    }
}
