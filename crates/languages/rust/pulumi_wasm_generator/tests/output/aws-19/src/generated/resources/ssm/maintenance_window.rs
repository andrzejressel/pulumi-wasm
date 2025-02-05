/// Provides an SSM Maintenance Window resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod maintenance_window {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MaintenanceWindowArgs {
        /// Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.
        #[builder(into, default)]
        pub allow_unassociated_targets: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.
        #[builder(into)]
        pub cutoff: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A description for the maintenance window.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The duration of the Maintenance Window in hours.
        #[builder(into)]
        pub duration: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Whether the maintenance window is enabled. Default: `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to no longer run the maintenance window.
        #[builder(into, default)]
        pub end_date: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the maintenance window.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The schedule of the Maintenance Window in the form of a [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html).
        #[builder(into)]
        pub schedule: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of days to wait after the date and time specified by a CRON expression before running the maintenance window.
        #[builder(into, default)]
        pub schedule_offset: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Timezone for schedule in [Internet Assigned Numbers Authority (IANA) Time Zone Database format](https://www.iana.org/time-zones). For example: `America/Los_Angeles`, `etc/UTC`, or `Asia/Seoul`.
        #[builder(into, default)]
        pub schedule_timezone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to begin the maintenance window.
        #[builder(into, default)]
        pub start_date: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MaintenanceWindowResult {
        /// Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.
        pub allow_unassociated_targets: pulumi_wasm_rust::Output<Option<bool>>,
        /// The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.
        pub cutoff: pulumi_wasm_rust::Output<i32>,
        /// A description for the maintenance window.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The duration of the Maintenance Window in hours.
        pub duration: pulumi_wasm_rust::Output<i32>,
        /// Whether the maintenance window is enabled. Default: `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to no longer run the maintenance window.
        pub end_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the maintenance window.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The schedule of the Maintenance Window in the form of a [cron or rate expression](https://docs.aws.amazon.com/systems-manager/latest/userguide/reference-cron-and-rate-expressions.html).
        pub schedule: pulumi_wasm_rust::Output<String>,
        /// The number of days to wait after the date and time specified by a CRON expression before running the maintenance window.
        pub schedule_offset: pulumi_wasm_rust::Output<Option<i32>>,
        /// Timezone for schedule in [Internet Assigned Numbers Authority (IANA) Time Zone Database format](https://www.iana.org/time-zones). For example: `America/Los_Angeles`, `etc/UTC`, or `Asia/Seoul`.
        pub schedule_timezone: pulumi_wasm_rust::Output<Option<String>>,
        /// Timestamp in [ISO-8601 extended format](https://www.iso.org/iso-8601-date-and-time-format.html) when to begin the maintenance window.
        pub start_date: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MaintenanceWindowArgs,
    ) -> MaintenanceWindowResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_unassociated_targets_binding = args
            .allow_unassociated_targets
            .get_output(context)
            .get_inner();
        let cutoff_binding = args.cutoff.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let duration_binding = args.duration.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let end_date_binding = args.end_date.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let schedule_binding = args.schedule.get_output(context).get_inner();
        let schedule_offset_binding = args
            .schedule_offset
            .get_output(context)
            .get_inner();
        let schedule_timezone_binding = args
            .schedule_timezone
            .get_output(context)
            .get_inner();
        let start_date_binding = args.start_date.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/maintenanceWindow:MaintenanceWindow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowUnassociatedTargets".into(),
                    value: &allow_unassociated_targets_binding,
                },
                register_interface::ObjectField {
                    name: "cutoff".into(),
                    value: &cutoff_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "endDate".into(),
                    value: &end_date_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleOffset".into(),
                    value: &schedule_offset_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleTimezone".into(),
                    value: &schedule_timezone_binding,
                },
                register_interface::ObjectField {
                    name: "startDate".into(),
                    value: &start_date_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MaintenanceWindowResult {
            allow_unassociated_targets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowUnassociatedTargets"),
            ),
            cutoff: pulumi_wasm_rust::__private::into_domain(o.extract_field("cutoff")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("duration"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            end_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endDate"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            schedule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("schedule"),
            ),
            schedule_offset: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scheduleOffset"),
            ),
            schedule_timezone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scheduleTimezone"),
            ),
            start_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("startDate"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
