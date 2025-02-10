/// Provides an SSM Maintenance Window resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let production = maintenance_window::create(
///         "production",
///         MaintenanceWindowArgs::builder()
///             .cutoff(1)
///             .duration(3)
///             .name("maintenance-window-application")
///             .schedule("cron(0 16 ? * TUE *)")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM  Maintenance Windows using the maintenance window `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/maintenanceWindow:MaintenanceWindow imported-window mw-0123456789
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod maintenance_window {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MaintenanceWindowArgs {
        /// Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.
        #[builder(into, default)]
        pub allow_unassociated_targets: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.
        #[builder(into)]
        pub cutoff: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A description for the maintenance window.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The duration of the Maintenance Window in hours.
        #[builder(into)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Whether the maintenance window is enabled. Default: `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to no longer run the maintenance window.
        #[builder(into, default)]
        pub end_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the maintenance window.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The schedule of the Maintenance Window in the form of a [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html).
        #[builder(into)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of days to wait after the date and time specified by a CRON expression before running the maintenance window.
        #[builder(into, default)]
        pub schedule_offset: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Timezone for schedule in [Internet Assigned Numbers Authority (IANA) Time Zone Database format](https://www.iana.org/time-zones). For example: `America/Los_Angeles`, `etc/UTC`, or `Asia/Seoul`.
        #[builder(into, default)]
        pub schedule_timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to begin the maintenance window.
        #[builder(into, default)]
        pub start_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MaintenanceWindowResult {
        /// Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.
        pub allow_unassociated_targets: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.
        pub cutoff: pulumi_gestalt_rust::Output<i32>,
        /// A description for the maintenance window.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The duration of the Maintenance Window in hours.
        pub duration: pulumi_gestalt_rust::Output<i32>,
        /// Whether the maintenance window is enabled. Default: `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to no longer run the maintenance window.
        pub end_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the maintenance window.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The schedule of the Maintenance Window in the form of a [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html).
        pub schedule: pulumi_gestalt_rust::Output<String>,
        /// The number of days to wait after the date and time specified by a CRON expression before running the maintenance window.
        pub schedule_offset: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Timezone for schedule in [Internet Assigned Numbers Authority (IANA) Time Zone Database format](https://www.iana.org/time-zones). For example: `America/Los_Angeles`, `etc/UTC`, or `Asia/Seoul`.
        pub schedule_timezone: pulumi_gestalt_rust::Output<Option<String>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to begin the maintenance window.
        pub start_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MaintenanceWindowArgs,
    ) -> MaintenanceWindowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_unassociated_targets_binding = args
            .allow_unassociated_targets
            .get_output(context);
        let cutoff_binding = args.cutoff.get_output(context);
        let description_binding = args.description.get_output(context);
        let duration_binding = args.duration.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let end_date_binding = args.end_date.get_output(context);
        let name_binding = args.name.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let schedule_offset_binding = args.schedule_offset.get_output(context);
        let schedule_timezone_binding = args.schedule_timezone.get_output(context);
        let start_date_binding = args.start_date.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/maintenanceWindow:MaintenanceWindow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowUnassociatedTargets".into(),
                    value: allow_unassociated_targets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cutoff".into(),
                    value: cutoff_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "duration".into(),
                    value: duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endDate".into(),
                    value: end_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleOffset".into(),
                    value: schedule_offset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleTimezone".into(),
                    value: schedule_timezone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startDate".into(),
                    value: start_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MaintenanceWindowResult {
            allow_unassociated_targets: o.get_field("allowUnassociatedTargets"),
            cutoff: o.get_field("cutoff"),
            description: o.get_field("description"),
            duration: o.get_field("duration"),
            enabled: o.get_field("enabled"),
            end_date: o.get_field("endDate"),
            name: o.get_field("name"),
            schedule: o.get_field("schedule"),
            schedule_offset: o.get_field("scheduleOffset"),
            schedule_timezone: o.get_field("scheduleTimezone"),
            start_date: o.get_field("startDate"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
