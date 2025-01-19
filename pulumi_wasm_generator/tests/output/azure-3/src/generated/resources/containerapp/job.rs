/// Manages a Container App Job.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// The ID of the Container App Environment in which to create the Container App Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        /// A `event_trigger_config` block as defined below.
        #[builder(into, default)]
        pub event_trigger_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobEventTriggerConfig>,
        >,
        /// A `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `manual_trigger_config` block as defined below.
        #[builder(into, default)]
        pub manual_trigger_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobManualTriggerConfig>,
        >,
        /// Specifies the name of the Container App Job resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `registry` blocks as defined below.
        #[builder(into, default)]
        pub registries: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerapp::JobRegistry>>,
        >,
        /// The maximum number of times a replica is allowed to retry.
        #[builder(into, default)]
        pub replica_retry_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum number of seconds a replica is allowed to run.
        #[builder(into)]
        pub replica_timeout_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The name of the resource group in which to create the Container App Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `schedule_trigger_config` block as defined below.
        ///
        /// > ** NOTE **: Only one of `manual_trigger_config`, `event_trigger_config` or `schedule_trigger_config` can be specified.
        #[builder(into, default)]
        pub schedule_trigger_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobScheduleTriggerConfig>,
        >,
        /// One or more `secret` blocks as defined below.
        #[builder(into, default)]
        pub secrets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerapp::JobSecret>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` block as defined below.
        #[builder(into)]
        pub template: pulumi_wasm_rust::Output<
            super::super::types::containerapp::JobTemplate,
        >,
        /// The name of the workload profile to use for the Container App Job.
        #[builder(into, default)]
        pub workload_profile_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// The ID of the Container App Environment in which to create the Container App Job. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        /// The endpoint for the Container App Job event stream.
        pub event_stream_endpoint: pulumi_wasm_rust::Output<String>,
        /// A `event_trigger_config` block as defined below.
        pub event_trigger_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobEventTriggerConfig>,
        >,
        /// A `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `manual_trigger_config` block as defined below.
        pub manual_trigger_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobManualTriggerConfig>,
        >,
        /// Specifies the name of the Container App Job resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of the Public IP Addresses which the Container App uses for outbound network access.
        pub outbound_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// One or more `registry` blocks as defined below.
        pub registries: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerapp::JobRegistry>>,
        >,
        /// The maximum number of times a replica is allowed to retry.
        pub replica_retry_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// The maximum number of seconds a replica is allowed to run.
        pub replica_timeout_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// The name of the resource group in which to create the Container App Job. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `schedule_trigger_config` block as defined below.
        ///
        /// > ** NOTE **: Only one of `manual_trigger_config`, `event_trigger_config` or `schedule_trigger_config` can be specified.
        pub schedule_trigger_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerapp::JobScheduleTriggerConfig>,
        >,
        /// One or more `secret` blocks as defined below.
        pub secrets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerapp::JobSecret>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `template` block as defined below.
        pub template: pulumi_wasm_rust::Output<
            super::super::types::containerapp::JobTemplate,
        >,
        /// The name of the workload profile to use for the Container App Job.
        pub workload_profile_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: JobArgs) -> JobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_inner();
        let event_trigger_config_binding = args.event_trigger_config.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let manual_trigger_config_binding = args.manual_trigger_config.get_inner();
        let name_binding = args.name.get_inner();
        let registries_binding = args.registries.get_inner();
        let replica_retry_limit_binding = args.replica_retry_limit.get_inner();
        let replica_timeout_in_seconds_binding = args
            .replica_timeout_in_seconds
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let schedule_trigger_config_binding = args.schedule_trigger_config.get_inner();
        let secrets_binding = args.secrets.get_inner();
        let tags_binding = args.tags.get_inner();
        let template_binding = args.template.get_inner();
        let workload_profile_name_binding = args.workload_profile_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "eventTriggerConfig".into(),
                    value: &event_trigger_config_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "manualTriggerConfig".into(),
                    value: &manual_trigger_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registries".into(),
                    value: &registries_binding,
                },
                register_interface::ObjectField {
                    name: "replicaRetryLimit".into(),
                    value: &replica_retry_limit_binding,
                },
                register_interface::ObjectField {
                    name: "replicaTimeoutInSeconds".into(),
                    value: &replica_timeout_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scheduleTriggerConfig".into(),
                    value: &schedule_trigger_config_binding,
                },
                register_interface::ObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "template".into(),
                    value: &template_binding,
                },
                register_interface::ObjectField {
                    name: "workloadProfileName".into(),
                    value: &workload_profile_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerAppEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "eventStreamEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "eventTriggerConfig".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "manualTriggerConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outboundIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "registries".into(),
                },
                register_interface::ResultField {
                    name: "replicaRetryLimit".into(),
                },
                register_interface::ResultField {
                    name: "replicaTimeoutInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scheduleTriggerConfig".into(),
                },
                register_interface::ResultField {
                    name: "secrets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "template".into(),
                },
                register_interface::ResultField {
                    name: "workloadProfileName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        JobResult {
            container_app_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAppEnvironmentId").unwrap(),
            ),
            event_stream_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventStreamEndpoint").unwrap(),
            ),
            event_trigger_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventTriggerConfig").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            manual_trigger_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manualTriggerConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outbound_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundIpAddresses").unwrap(),
            ),
            registries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registries").unwrap(),
            ),
            replica_retry_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaRetryLimit").unwrap(),
            ),
            replica_timeout_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicaTimeoutInSeconds").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            schedule_trigger_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduleTriggerConfig").unwrap(),
            ),
            secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secrets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("template").unwrap(),
            ),
            workload_profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadProfileName").unwrap(),
            ),
        }
    }
}
