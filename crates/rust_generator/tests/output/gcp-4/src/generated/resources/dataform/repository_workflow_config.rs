/// ## Example Usage
///
/// ### Dataform Repository Workflow Config
///
///
/// ```yaml
/// resources:
///   gitRepository:
///     type: gcp:sourcerepo:Repository
///     name: git_repository
///     properties:
///       name: my/repository
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: my_secret
///       replication:
///         auto: {}
///   secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: secret_version
///     properties:
///       secret: ${secret.id}
///       secretData: secret-data
///   repository:
///     type: gcp:dataform:Repository
///     properties:
///       name: dataform_repository
///       region: us-central1
///       gitRemoteSettings:
///         url: ${gitRepository.url}
///         defaultBranch: main
///         authenticationTokenSecretVersion: ${secretVersion.id}
///       workspaceCompilationOverrides:
///         defaultDatabase: database
///         schemaSuffix: _suffix
///         tablePrefix: prefix_
///   releaseConfig:
///     type: gcp:dataform:RepositoryReleaseConfig
///     name: release_config
///     properties:
///       project: ${repository.project}
///       region: ${repository.region}
///       repository: ${repository.name}
///       name: my_release
///       gitCommitish: main
///       cronSchedule: 0 7 * * *
///       timeZone: America/New_York
///       codeCompilationConfig:
///         defaultDatabase: gcp-example-project
///         defaultSchema: example-dataset
///         defaultLocation: us-central1
///         assertionSchema: example-assertion-dataset
///         databaseSuffix: ""
///         schemaSuffix: ""
///         tablePrefix: ""
///         vars:
///           var1: value
///   dataformSa:
///     type: gcp:serviceaccount:Account
///     name: dataform_sa
///     properties:
///       accountId: dataform-sa
///       displayName: Dataform Service Account
///   workflow:
///     type: gcp:dataform:RepositoryWorkflowConfig
///     properties:
///       project: ${repository.project}
///       region: ${repository.region}
///       repository: ${repository.name}
///       name: my_workflow
///       releaseConfig: ${releaseConfig.id}
///       invocationConfig:
///         includedTargets:
///           - database: gcp-example-project
///             schema: example-dataset
///             name: target_1
///           - database: gcp-example-project
///             schema: example-dataset
///             name: target_2
///         includedTags:
///           - tag_1
///         transitiveDependenciesIncluded: true
///         transitiveDependentsIncluded: true
///         fullyRefreshIncrementalTablesEnabled: false
///         serviceAccount: ${dataformSa.email}
///       cronSchedule: 0 7 * * *
///       timeZone: America/New_York
/// ```
///
/// ## Import
///
/// RepositoryWorkflowConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/repositories/{{repository}}/workflowConfigs/{{name}}`
///
/// * `{{project}}/{{region}}/{{repository}}/{{name}}`
///
/// * `{{region}}/{{repository}}/{{name}}`
///
/// * `{{repository}}/{{name}}`
///
/// When using the `pulumi import` command, RepositoryWorkflowConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default projects/{{project}}/locations/{{region}}/repositories/{{repository}}/workflowConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default {{project}}/{{region}}/{{repository}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default {{region}}/{{repository}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default {{repository}}/{{name}}
/// ```
///
pub mod repository_workflow_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryWorkflowConfigArgs {
        /// Optional. Optional schedule (in cron format) for automatic creation of compilation results.
        #[builder(into, default)]
        pub cron_schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. If left unset, a default InvocationConfig will be used.
        /// Structure is documented below.
        #[builder(into, default)]
        pub invocation_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataform::RepositoryWorkflowConfigInvocationConfig,
            >,
        >,
        /// The workflow's name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the release config whose releaseCompilationResult should be executed. Must be in the format projects/*/locations/*/repositories/*/releaseConfigs/*.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub release_config: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A reference to the Dataform repository
        #[builder(into, default)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Specifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RepositoryWorkflowConfigResult {
        /// Optional. Optional schedule (in cron format) for automatic creation of compilation results.
        pub cron_schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. If left unset, a default InvocationConfig will be used.
        /// Structure is documented below.
        pub invocation_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataform::RepositoryWorkflowConfigInvocationConfig,
            >,
        >,
        /// The workflow's name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Records of the 10 most recent scheduled execution attempts, ordered in in descending order of executionTime. Updated whenever automatic creation of a workflow invocation is triggered by cronSchedule.
        /// Structure is documented below.
        pub recent_scheduled_execution_records: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::dataform::RepositoryWorkflowConfigRecentScheduledExecutionRecord,
            >,
        >,
        /// A reference to the region
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the release config whose releaseCompilationResult should be executed. Must be in the format projects/*/locations/*/repositories/*/releaseConfigs/*.
        ///
        ///
        /// - - -
        pub release_config: pulumi_gestalt_rust::Output<String>,
        /// A reference to the Dataform repository
        pub repository: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Specifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC.
        pub time_zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RepositoryWorkflowConfigArgs,
    ) -> RepositoryWorkflowConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cron_schedule_binding = args.cron_schedule.get_output(context).get_inner();
        let invocation_config_binding = args
            .invocation_config
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let release_config_binding = args.release_config.get_output(context).get_inner();
        let repository_binding = args.repository.get_output(context).get_inner();
        let time_zone_binding = args.time_zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cronSchedule".into(),
                    value: &cron_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "invocationConfig".into(),
                    value: &invocation_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "releaseConfig".into(),
                    value: &release_config_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryWorkflowConfigResult {
            cron_schedule: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cronSchedule"),
            ),
            invocation_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invocationConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            recent_scheduled_execution_records: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recentScheduledExecutionRecords"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            release_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseConfig"),
            ),
            repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repository"),
            ),
            time_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeZone"),
            ),
        }
    }
}
