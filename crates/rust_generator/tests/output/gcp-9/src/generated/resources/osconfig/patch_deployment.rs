/// Patch deployments are configurations that individual patch jobs use to complete a patch.
/// These configurations include instance filter, package repository settings, and a schedule.
///
///
/// To get more information about PatchDeployment, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/osconfig/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/os-patch-management)
///
/// ## Example Usage
///
/// ### Os Config Patch Deployment Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let patch = patch_deployment::create(
///         "patch",
///         PatchDeploymentArgs::builder()
///             .instance_filter(
///                 PatchDeploymentInstanceFilter::builder().all(true).build_struct(),
///             )
///             .one_time_schedule(
///                 PatchDeploymentOneTimeSchedule::builder()
///                     .executeTime("2999-10-10T10:10:10.045123456Z")
///                     .build_struct(),
///             )
///             .patch_deployment_id("patch-deploy")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Os Config Patch Deployment Daily
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let patch = patch_deployment::create(
///         "patch",
///         PatchDeploymentArgs::builder()
///             .instance_filter(
///                 PatchDeploymentInstanceFilter::builder().all(true).build_struct(),
///             )
///             .patch_deployment_id("patch-deploy")
///             .recurring_schedule(
///                 PatchDeploymentRecurringSchedule::builder()
///                     .timeOfDay(
///                         PatchDeploymentRecurringScheduleTimeOfDay::builder()
///                             .hours(0)
///                             .minutes(30)
///                             .nanos(20)
///                             .seconds(30)
///                             .build_struct(),
///                     )
///                     .timeZone(
///                         PatchDeploymentRecurringScheduleTimeZone::builder()
///                             .id("America/New_York")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Os Config Patch Deployment Daily Midnight
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let patch = patch_deployment::create(
///         "patch",
///         PatchDeploymentArgs::builder()
///             .instance_filter(
///                 PatchDeploymentInstanceFilter::builder().all(true).build_struct(),
///             )
///             .patch_deployment_id("patch-deploy")
///             .recurring_schedule(
///                 PatchDeploymentRecurringSchedule::builder()
///                     .timeOfDay(
///                         PatchDeploymentRecurringScheduleTimeOfDay::builder()
///                             .hours(0)
///                             .minutes(0)
///                             .nanos(0)
///                             .seconds(0)
///                             .build_struct(),
///                     )
///                     .timeZone(
///                         PatchDeploymentRecurringScheduleTimeZone::builder()
///                             .id("America/New_York")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Os Config Patch Deployment Instance
///
///
/// ```yaml
/// resources:
///   foobar:
///     type: gcp:compute:Instance
///     properties:
///       name: patch-deploy-inst
///       machineType: e2-medium
///       zone: us-central1-a
///       canIpForward: false
///       tags:
///         - foo
///         - bar
///       bootDisk:
///         initializeParams:
///           image: ${myImage.selfLink}
///       networkInterfaces:
///         - network: default
///       metadata:
///         foo: bar
///   patch:
///     type: gcp:osconfig:PatchDeployment
///     properties:
///       patchDeploymentId: patch-deploy
///       instanceFilter:
///         instances:
///           - ${foobar.id}
///       patchConfig:
///         yum:
///           security: true
///           minimal: true
///           excludes:
///             - bash
///       recurringSchedule:
///         timeZone:
///           id: America/New_York
///         timeOfDay:
///           hours: 0
///           minutes: 30
///           seconds: 30
///           nanos: 20
///         monthly:
///           monthDay: 1
/// variables:
///   myImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Os Config Patch Deployment Full
///
///
/// ```yaml
/// resources:
///   patch:
///     type: gcp:osconfig:PatchDeployment
///     properties:
///       patchDeploymentId: patch-deploy
///       instanceFilter:
///         groupLabels:
///           - labels:
///               env: dev
///               app: web
///         instanceNamePrefixes:
///           - test-
///         zones:
///           - us-central1-a
///           - us-central-1c
///       patchConfig:
///         migInstancesAllowed: true
///         rebootConfig: ALWAYS
///         apt:
///           type: DIST
///           excludes:
///             - python
///         yum:
///           security: true
///           minimal: true
///           excludes:
///             - bash
///         goo:
///           enabled: true
///         zypper:
///           categories:
///             - security
///         windowsUpdate:
///           classifications:
///             - CRITICAL
///             - SECURITY
///             - UPDATE
///           excludes:
///             - '5012170'
///         preStep:
///           linuxExecStepConfig:
///             allowedSuccessCodes:
///               - 0
///               - 3
///             localPath: /tmp/pre_patch_script.sh
///           windowsExecStepConfig:
///             interpreter: SHELL
///             allowedSuccessCodes:
///               - 0
///               - 2
///             localPath: C:\Users\user\pre-patch-script.cmd
///         postStep:
///           linuxExecStepConfig:
///             gcsObject:
///               bucket: my-patch-scripts
///               generationNumber: '1523477886880'
///               object: linux/post_patch_script
///           windowsExecStepConfig:
///             interpreter: POWERSHELL
///             gcsObject:
///               bucket: my-patch-scripts
///               generationNumber: '135920493447'
///               object: windows/post_patch_script.ps1
///       duration: 10s
///       recurringSchedule:
///         timeZone:
///           id: America/New_York
///         timeOfDay:
///           hours: 0
///           minutes: 30
///           seconds: 30
///           nanos: 20
///         monthly:
///           weekDayOfMonth:
///             weekOrdinal: -1
///             dayOfWeek: TUESDAY
///             dayOffset: 3
///       rollout:
///         mode: ZONE_BY_ZONE
///         disruptionBudget:
///           fixed: 1
/// ```
///
/// ## Import
///
/// PatchDeployment can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PatchDeployment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:osconfig/patchDeployment:PatchDeployment default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:osconfig/patchDeployment:PatchDeployment default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:osconfig/patchDeployment:PatchDeployment default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod patch_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PatchDeploymentArgs {
        /// Description of the patch deployment. Length of the description is limited to 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Duration of the patch. After the duration ends, the patch times out. A duration in seconds with up to nine fractional
        /// digits, terminated by 's'. Example: "3.5s"
        #[builder(into, default)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VM instances to patch.
        /// Structure is documented below.
        #[builder(into)]
        pub instance_filter: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::osconfig::PatchDeploymentInstanceFilter,
        >,
        /// Schedule a one-time execution.
        #[builder(into, default)]
        pub one_time_schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::osconfig::PatchDeploymentOneTimeSchedule>,
        >,
        /// Patch configuration that is applied.
        #[builder(into, default)]
        pub patch_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::osconfig::PatchDeploymentPatchConfig>,
        >,
        /// A name for the patch deployment in the project. When creating a name the following rules apply:
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the project.
        #[builder(into)]
        pub patch_deployment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Schedule recurring executions.
        #[builder(into, default)]
        pub recurring_schedule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::osconfig::PatchDeploymentRecurringSchedule>,
        >,
        /// Rollout strategy of the patch job.
        #[builder(into, default)]
        pub rollout: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::osconfig::PatchDeploymentRollout>,
        >,
    }
    #[allow(dead_code)]
    pub struct PatchDeploymentResult {
        /// Time the patch deployment was created. Timestamp is in RFC3339 text format.
        /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the patch deployment. Length of the description is limited to 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Duration of the patch. After the duration ends, the patch times out. A duration in seconds with up to nine fractional
        /// digits, terminated by 's'. Example: "3.5s"
        pub duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// VM instances to patch.
        /// Structure is documented below.
        pub instance_filter: pulumi_gestalt_rust::Output<
            super::super::types::osconfig::PatchDeploymentInstanceFilter,
        >,
        /// The last time a patch job was started by this deployment. Timestamp is in RFC3339 text format.
        /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub last_execute_time: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the patch deployment resource in a project.
        /// The patch deployment name is in the form: projects/{project_id}/patchDeployments/{patchDeploymentId}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Schedule a one-time execution.
        pub one_time_schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::osconfig::PatchDeploymentOneTimeSchedule>,
        >,
        /// Patch configuration that is applied.
        pub patch_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::osconfig::PatchDeploymentPatchConfig>,
        >,
        /// A name for the patch deployment in the project. When creating a name the following rules apply:
        /// * Must contain only lowercase letters, numbers, and hyphens.
        /// * Must start with a letter.
        /// * Must be between 1-63 characters.
        /// * Must end with a number or a letter.
        /// * Must be unique within the project.
        pub patch_deployment_id: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Schedule recurring executions.
        pub recurring_schedule: pulumi_gestalt_rust::Output<
            Option<super::super::types::osconfig::PatchDeploymentRecurringSchedule>,
        >,
        /// Rollout strategy of the patch job.
        pub rollout: pulumi_gestalt_rust::Output<
            Option<super::super::types::osconfig::PatchDeploymentRollout>,
        >,
        /// Time the patch deployment was last updated. Timestamp is in RFC3339 text format.
        /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PatchDeploymentArgs,
    ) -> PatchDeploymentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let duration_binding = args.duration.get_output(context).get_inner();
        let instance_filter_binding = args
            .instance_filter
            .get_output(context)
            .get_inner();
        let one_time_schedule_binding = args
            .one_time_schedule
            .get_output(context)
            .get_inner();
        let patch_config_binding = args.patch_config.get_output(context).get_inner();
        let patch_deployment_id_binding = args
            .patch_deployment_id
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let recurring_schedule_binding = args
            .recurring_schedule
            .get_output(context)
            .get_inner();
        let rollout_binding = args.rollout.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:osconfig/patchDeployment:PatchDeployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "instanceFilter".into(),
                    value: &instance_filter_binding,
                },
                register_interface::ObjectField {
                    name: "oneTimeSchedule".into(),
                    value: &one_time_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "patchConfig".into(),
                    value: &patch_config_binding,
                },
                register_interface::ObjectField {
                    name: "patchDeploymentId".into(),
                    value: &patch_deployment_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "recurringSchedule".into(),
                    value: &recurring_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "rollout".into(),
                    value: &rollout_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PatchDeploymentResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("duration"),
            ),
            instance_filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceFilter"),
            ),
            last_execute_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastExecuteTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            one_time_schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("oneTimeSchedule"),
            ),
            patch_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("patchConfig"),
            ),
            patch_deployment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("patchDeploymentId"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            recurring_schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recurringSchedule"),
            ),
            rollout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rollout"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
