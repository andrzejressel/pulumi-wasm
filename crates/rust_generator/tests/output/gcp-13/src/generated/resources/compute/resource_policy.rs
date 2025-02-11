/// A policy that can be attached to a resource to specify or schedule actions on that resource.
///
///
/// To get more information about ResourcePolicy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/resourcePolicies)
///
/// ## Example Usage
///
/// ### Resource Policy Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = resource_policy::create(
///         "foo",
///         ResourcePolicyArgs::builder()
///             .name("gce-policy")
///             .region("us-central1")
///             .snapshot_schedule_policy(
///                 ResourcePolicySnapshotSchedulePolicy::builder()
///                     .schedule(
///                         ResourcePolicySnapshotSchedulePolicySchedule::builder()
///                             .dailySchedule(
///                                 ResourcePolicySnapshotSchedulePolicyScheduleDailySchedule::builder()
///                                     .daysInCycle(1)
///                                     .startTime("04:00")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Resource Policy Full
///
///
/// ```yaml
/// resources:
///   bar:
///     type: gcp:compute:ResourcePolicy
///     properties:
///       name: gce-policy
///       region: us-central1
///       snapshotSchedulePolicy:
///         schedule:
///           hourlySchedule:
///             hoursInCycle: 20
///             startTime: 23:00
///         retentionPolicy:
///           maxRetentionDays: 10
///           onSourceDiskDelete: KEEP_AUTO_SNAPSHOTS
///         snapshotProperties:
///           labels:
///             my_label: value
///           storageLocations: us
///           guestFlush: true
/// ```
/// ### Resource Policy Placement Policy
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let baz = resource_policy::create(
///         "baz",
///         ResourcePolicyArgs::builder()
///             .group_placement_policy(
///                 ResourcePolicyGroupPlacementPolicy::builder()
///                     .collocation("COLLOCATED")
///                     .vmCount(2)
///                     .build_struct(),
///             )
///             .name("gce-policy")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Resource Policy Placement Policy Max Distance
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let baz = resource_policy::create(
///         "baz",
///         ResourcePolicyArgs::builder()
///             .group_placement_policy(
///                 ResourcePolicyGroupPlacementPolicy::builder()
///                     .collocation("COLLOCATED")
///                     .maxDistance(2)
///                     .vmCount(2)
///                     .build_struct(),
///             )
///             .name("gce-policy")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Resource Policy Instance Schedule Policy
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let hourly = resource_policy::create(
///         "hourly",
///         ResourcePolicyArgs::builder()
///             .description("Start and stop instances")
///             .instance_schedule_policy(
///                 ResourcePolicyInstanceSchedulePolicy::builder()
///                     .timeZone("US/Central")
///                     .vmStartSchedule(
///                         ResourcePolicyInstanceSchedulePolicyVmStartSchedule::builder()
///                             .schedule("0 * * * *")
///                             .build_struct(),
///                     )
///                     .vmStopSchedule(
///                         ResourcePolicyInstanceSchedulePolicyVmStopSchedule::builder()
///                             .schedule("15 * * * *")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("gce-policy")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Resource Policy Snapshot Schedule Chain Name
///
///
/// ```yaml
/// resources:
///   hourly:
///     type: gcp:compute:ResourcePolicy
///     properties:
///       name: gce-policy
///       region: us-central1
///       description: chain name snapshot
///       snapshotSchedulePolicy:
///         schedule:
///           hourlySchedule:
///             hoursInCycle: 20
///             startTime: 23:00
///         retentionPolicy:
///           maxRetentionDays: 14
///           onSourceDiskDelete: KEEP_AUTO_SNAPSHOTS
///         snapshotProperties:
///           labels:
///             my_label: value
///           storageLocations: us
///           guestFlush: true
///           chainName: test-schedule-chain-name
/// ```
/// ### Resource Policy Consistency Group
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cgroup = resource_policy::create(
///         "cgroup",
///         ResourcePolicyArgs::builder()
///             .disk_consistency_group_policy(
///                 ResourcePolicyDiskConsistencyGroupPolicy::builder()
///                     .enabled(true)
///                     .build_struct(),
///             )
///             .name("gce-policy")
///             .region("europe-west1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ResourcePolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/resourcePolicies/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ResourcePolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/resourcePolicy:ResourcePolicy default projects/{{project}}/regions/{{region}}/resourcePolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/resourcePolicy:ResourcePolicy default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/resourcePolicy:ResourcePolicy default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/resourcePolicy:ResourcePolicy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Replication consistency group for asynchronous disk replication.
        /// Structure is documented below.
        #[builder(into, default)]
        pub disk_consistency_group_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::ResourcePolicyDiskConsistencyGroupPolicy,
            >,
        >,
        /// Resource policy for instances used for placement configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub group_placement_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ResourcePolicyGroupPlacementPolicy>,
        >,
        /// Resource policy for scheduling instance operations.
        /// Structure is documented below.
        #[builder(into, default)]
        pub instance_schedule_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ResourcePolicyInstanceSchedulePolicy>,
        >,
        /// The name of the resource, provided by the client when initially creating
        /// the resource. The resource name must be 1-63 characters long, and comply
        /// with RFC1035. Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z`? which means the
        /// first character must be a lowercase letter, and all following characters
        /// must be a dash, lowercase letter, or digit, except the last character,
        /// which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where resource policy resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy for creating snapshots of persistent disks.
        /// Structure is documented below.
        #[builder(into, default)]
        pub snapshot_schedule_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ResourcePolicySnapshotSchedulePolicy>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Replication consistency group for asynchronous disk replication.
        /// Structure is documented below.
        pub disk_consistency_group_policy: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::ResourcePolicyDiskConsistencyGroupPolicy,
            >,
        >,
        /// Resource policy for instances used for placement configuration.
        /// Structure is documented below.
        pub group_placement_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ResourcePolicyGroupPlacementPolicy>,
        >,
        /// Resource policy for scheduling instance operations.
        /// Structure is documented below.
        pub instance_schedule_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ResourcePolicyInstanceSchedulePolicy>,
        >,
        /// The name of the resource, provided by the client when initially creating
        /// the resource. The resource name must be 1-63 characters long, and comply
        /// with RFC1035. Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z`? which means the
        /// first character must be a lowercase letter, and all following characters
        /// must be a dash, lowercase letter, or digit, except the last character,
        /// which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Region where resource policy resides.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Policy for creating snapshots of persistent disks.
        /// Structure is documented below.
        pub snapshot_schedule_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::ResourcePolicySnapshotSchedulePolicy>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResourcePolicyArgs,
    ) -> ResourcePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let disk_consistency_group_policy_binding = args
            .disk_consistency_group_policy
            .get_output(context);
        let group_placement_policy_binding = args
            .group_placement_policy
            .get_output(context);
        let instance_schedule_policy_binding = args
            .instance_schedule_policy
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let snapshot_schedule_policy_binding = args
            .snapshot_schedule_policy
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskConsistencyGroupPolicy".into(),
                    value: &disk_consistency_group_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupPlacementPolicy".into(),
                    value: &group_placement_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceSchedulePolicy".into(),
                    value: &instance_schedule_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotSchedulePolicy".into(),
                    value: &snapshot_schedule_policy_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourcePolicyResult {
            description: o.get_field("description"),
            disk_consistency_group_policy: o.get_field("diskConsistencyGroupPolicy"),
            group_placement_policy: o.get_field("groupPlacementPolicy"),
            instance_schedule_policy: o.get_field("instanceSchedulePolicy"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            snapshot_schedule_policy: o.get_field("snapshotSchedulePolicy"),
        }
    }
}
