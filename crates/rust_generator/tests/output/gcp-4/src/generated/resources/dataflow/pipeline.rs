/// The main pipeline entity and all the necessary metadata for launching and managing linked jobs.
///
///
/// To get more information about Pipeline, see:
///
/// * [API documentation](https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dataflow)
///
/// ## Example Usage
///
/// ### Data Pipeline Pipeline
///
///
/// ```yaml
/// resources:
///   serviceAccount:
///     type: gcp:serviceaccount:Account
///     name: service_account
///     properties:
///       accountId: my-account
///       displayName: Service Account
///   primary:
///     type: gcp:dataflow:Pipeline
///     properties:
///       name: my-pipeline
///       displayName: my-pipeline
///       type: PIPELINE_TYPE_BATCH
///       state: STATE_ACTIVE
///       region: us-central1
///       workload:
///         dataflowLaunchTemplateRequest:
///           projectId: my-project
///           gcsPath: gs://my-bucket/path
///           launchParameters:
///             jobName: my-job
///             parameters:
///               name: wrench
///             environment:
///               numWorkers: 5
///               maxWorkers: 5
///               zone: us-centra1-a
///               serviceAccountEmail: ${serviceAccount.email}
///               network: default
///               tempLocation: gs://my-bucket/tmp_dir
///               bypassTempDirValidation: false
///               machineType: E2
///               additionalUserLabels:
///                 context: test
///               workerRegion: us-central1
///               workerZone: us-central1-a
///               enableStreamingEngine: 'false'
///             update: false
///             transformNameMapping:
///               name: wrench
///           location: us-central1
///       scheduleInfo:
///         schedule: '* */2 * * *'
/// ```
///
/// ## Import
///
/// Pipeline can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/pipelines/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Pipeline can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataflow/pipeline:Pipeline default projects/{{project}}/locations/{{region}}/pipelines/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataflow/pipeline:Pipeline default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataflow/pipeline:Pipeline default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataflow/pipeline:Pipeline default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pipeline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PipelineArgs {
        /// The display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_).
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// "The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID."
        /// "- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects."
        /// "LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions."
        /// "PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location."
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        #[builder(into, default)]
        pub pipeline_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Internal scheduling information for a pipeline. If this information is provided, periodic jobs will be created per the schedule. If not, users are responsible for creating jobs externally.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#schedulespec
        /// Structure is documented below.
        #[builder(into, default)]
        pub schedule_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataflow::PipelineScheduleInfo>,
        >,
        /// Optional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used.
        #[builder(into, default)]
        pub scheduler_service_account_email: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state
        /// Possible values are: `STATE_UNSPECIFIED`, `STATE_RESUMING`, `STATE_ACTIVE`, `STATE_STOPPING`, `STATE_ARCHIVED`, `STATE_PAUSED`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype
        /// Possible values are: `PIPELINE_TYPE_UNSPECIFIED`, `PIPELINE_TYPE_BATCH`, `PIPELINE_TYPE_STREAMING`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Workload information for creating new jobs.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#workload
        /// Structure is documented below.
        #[builder(into, default)]
        pub workload: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataflow::PipelineWorkload>,
        >,
    }
    #[allow(dead_code)]
    pub struct PipelineResult {
        /// The timestamp when the pipeline was initially created. Set by the Data Pipelines service.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_).
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Number of jobs.
        pub job_count: pulumi_gestalt_rust::Output<i32>,
        /// The timestamp when the pipeline was last modified. Set by the Data Pipelines service.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub last_update_time: pulumi_gestalt_rust::Output<String>,
        /// "The pipeline name. For example': 'projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID."
        /// "- PROJECT_ID can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see Identifying projects."
        /// "LOCATION_ID is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling google.cloud.location.Locations.ListLocations. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in App Engine regions."
        /// "PIPELINE_ID is the ID of the pipeline. Must be unique for the selected project and location."
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        pub pipeline_sources: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A reference to the region
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Internal scheduling information for a pipeline. If this information is provided, periodic jobs will be created per the schedule. If not, users are responsible for creating jobs externally.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#schedulespec
        /// Structure is documented below.
        pub schedule_info: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataflow::PipelineScheduleInfo>,
        >,
        /// Optional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used.
        pub scheduler_service_account_email: pulumi_gestalt_rust::Output<String>,
        /// The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through pipelines.patch requests.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#state
        /// Possible values are: `STATE_UNSPECIFIED`, `STATE_RESUMING`, `STATE_ACTIVE`, `STATE_STOPPING`, `STATE_ARCHIVED`, `STATE_PAUSED`.
        ///
        ///
        /// - - -
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#pipelinetype
        /// Possible values are: `PIPELINE_TYPE_UNSPECIFIED`, `PIPELINE_TYPE_BATCH`, `PIPELINE_TYPE_STREAMING`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Workload information for creating new jobs.
        /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#workload
        /// Structure is documented below.
        pub workload: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataflow::PipelineWorkload>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PipelineArgs,
    ) -> PipelineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let pipeline_sources_binding = args.pipeline_sources.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let schedule_info_binding = args.schedule_info.get_output(context);
        let scheduler_service_account_email_binding = args
            .scheduler_service_account_email
            .get_output(context);
        let state_binding = args.state.get_output(context);
        let type__binding = args.type_.get_output(context);
        let workload_binding = args.workload.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataflow/pipeline:Pipeline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineSources".into(),
                    value: pipeline_sources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleInfo".into(),
                    value: schedule_info_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedulerServiceAccountEmail".into(),
                    value: scheduler_service_account_email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workload".into(),
                    value: workload_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PipelineResult {
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            job_count: o.get_field("jobCount"),
            last_update_time: o.get_field("lastUpdateTime"),
            name: o.get_field("name"),
            pipeline_sources: o.get_field("pipelineSources"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            schedule_info: o.get_field("scheduleInfo"),
            scheduler_service_account_email: o.get_field("schedulerServiceAccountEmail"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            workload: o.get_field("workload"),
        }
    }
}
