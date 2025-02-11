/// Manages a Container App Job.
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
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-log-analytics-workspace")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleEnvironment = environment::create(
///         "exampleEnvironment",
///         EnvironmentArgs::builder()
///             .location("${example.location}")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("example-container-app-environment")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleJob = job::create(
///         "exampleJob",
///         JobArgs::builder()
///             .container_app_environment_id("${exampleEnvironment.id}")
///             .location("${example.location}")
///             .manual_trigger_config(
///                 JobManualTriggerConfig::builder()
///                     .parallelism(4)
///                     .replicaCompletionCount(1)
///                     .build_struct(),
///             )
///             .name("example-container-app-job")
///             .replica_retry_limit(10)
///             .replica_timeout_in_seconds(10)
///             .resource_group_name("${example.name}")
///             .template(
///                 JobTemplate::builder()
///                     .containers(
///                         vec![
///                             JobTemplateContainer::builder().cpu(0.5)
///                             .image("repo/testcontainerAppsJob0:v1")
///                             .livenessProbes(vec![JobTemplateContainerLivenessProbe::builder()
///                             .failureCountThreshold(1)
///                             .headers(vec![JobTemplateContainerLivenessProbeHeader::builder()
///                             .name("Cache-Control").value("no-cache").build_struct(),])
///                             .initialDelay(5).intervalSeconds(20).path("/health")
///                             .port(5000).timeout(2).transport("HTTP").build_struct(),])
///                             .memory("1Gi").name("testcontainerappsjob0")
///                             .readinessProbes(vec![JobTemplateContainerReadinessProbe::builder()
///                             .port(5000).transport("HTTP").build_struct(),])
///                             .startupProbes(vec![JobTemplateContainerStartupProbe::builder()
///                             .port(5000).transport("TCP").build_struct(),])
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// A Container App Job can be imported using the resource id, e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/job:Job example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.App/jobs/example-container-app-job"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// The ID of the Container App Environment in which to create the Container App Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `event_trigger_config` block as defined below.
        #[builder(into, default)]
        pub event_trigger_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerapp::JobEventTriggerConfig>,
        >,
        /// A `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerapp::JobIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `manual_trigger_config` block as defined below.
        #[builder(into, default)]
        pub manual_trigger_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerapp::JobManualTriggerConfig>,
        >,
        /// Specifies the name of the Container App Job resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `registry` blocks as defined below.
        #[builder(into, default)]
        pub registries: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerapp::JobRegistry>>,
        >,
        /// The maximum number of times a replica is allowed to retry.
        #[builder(into, default)]
        pub replica_retry_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum number of seconds a replica is allowed to run.
        #[builder(into)]
        pub replica_timeout_in_seconds: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The name of the resource group in which to create the Container App Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `schedule_trigger_config` block as defined below.
        ///
        /// > ** NOTE **: Only one of `manual_trigger_config`, `event_trigger_config` or `schedule_trigger_config` can be specified.
        #[builder(into, default)]
        pub schedule_trigger_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerapp::JobScheduleTriggerConfig>,
        >,
        /// One or more `secret` blocks as defined below.
        #[builder(into, default)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerapp::JobSecret>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` block as defined below.
        #[builder(into)]
        pub template: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::containerapp::JobTemplate,
        >,
        /// The name of the workload profile to use for the Container App Job.
        #[builder(into, default)]
        pub workload_profile_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// The ID of the Container App Environment in which to create the Container App Job. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_gestalt_rust::Output<String>,
        /// The endpoint for the Container App Job event stream.
        pub event_stream_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A `event_trigger_config` block as defined below.
        pub event_trigger_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerapp::JobEventTriggerConfig>,
        >,
        /// A `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerapp::JobIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `manual_trigger_config` block as defined below.
        pub manual_trigger_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerapp::JobManualTriggerConfig>,
        >,
        /// Specifies the name of the Container App Job resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of the Public IP Addresses which the Container App uses for outbound network access.
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// One or more `registry` blocks as defined below.
        pub registries: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerapp::JobRegistry>>,
        >,
        /// The maximum number of times a replica is allowed to retry.
        pub replica_retry_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum number of seconds a replica is allowed to run.
        pub replica_timeout_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// The name of the resource group in which to create the Container App Job. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `schedule_trigger_config` block as defined below.
        ///
        /// > ** NOTE **: Only one of `manual_trigger_config`, `event_trigger_config` or `schedule_trigger_config` can be specified.
        pub schedule_trigger_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerapp::JobScheduleTriggerConfig>,
        >,
        /// One or more `secret` blocks as defined below.
        pub secrets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerapp::JobSecret>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` block as defined below.
        pub template: pulumi_gestalt_rust::Output<
            super::super::types::containerapp::JobTemplate,
        >,
        /// The name of the workload profile to use for the Container App Job.
        pub workload_profile_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobArgs,
    ) -> JobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_output(context);
        let event_trigger_config_binding = args.event_trigger_config.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let manual_trigger_config_binding = args
            .manual_trigger_config
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let registries_binding = args.registries.get_output(context);
        let replica_retry_limit_binding = args.replica_retry_limit.get_output(context);
        let replica_timeout_in_seconds_binding = args
            .replica_timeout_in_seconds
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let schedule_trigger_config_binding = args
            .schedule_trigger_config
            .get_output(context);
        let secrets_binding = args.secrets.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_binding = args.template.get_output(context);
        let workload_profile_name_binding = args
            .workload_profile_name
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerapp/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventTriggerConfig".into(),
                    value: &event_trigger_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manualTriggerConfig".into(),
                    value: &manual_trigger_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registries".into(),
                    value: &registries_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicaRetryLimit".into(),
                    value: &replica_retry_limit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicaTimeoutInSeconds".into(),
                    value: &replica_timeout_in_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleTriggerConfig".into(),
                    value: &schedule_trigger_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "template".into(),
                    value: &template_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadProfileName".into(),
                    value: &workload_profile_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobResult {
            container_app_environment_id: o.get_field("containerAppEnvironmentId"),
            event_stream_endpoint: o.get_field("eventStreamEndpoint"),
            event_trigger_config: o.get_field("eventTriggerConfig"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            manual_trigger_config: o.get_field("manualTriggerConfig"),
            name: o.get_field("name"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            registries: o.get_field("registries"),
            replica_retry_limit: o.get_field("replicaRetryLimit"),
            replica_timeout_in_seconds: o.get_field("replicaTimeoutInSeconds"),
            resource_group_name: o.get_field("resourceGroupName"),
            schedule_trigger_config: o.get_field("scheduleTriggerConfig"),
            secrets: o.get_field("secrets"),
            tags: o.get_field("tags"),
            template: o.get_field("template"),
            workload_profile_name: o.get_field("workloadProfileName"),
        }
    }
}
