#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_trigger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTriggerArgs {
        /// The Cloud Build location for the trigger.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique identifier for the trigger..
        #[builder(into)]
        pub trigger_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTriggerResult {
        pub approval_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerApprovalConfig>,
        >,
        pub bitbucket_server_trigger_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudbuild::GetTriggerBitbucketServerTriggerConfig,
            >,
        >,
        pub builds: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerBuild>,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disabled: pulumi_gestalt_rust::Output<bool>,
        pub filename: pulumi_gestalt_rust::Output<String>,
        pub filter: pulumi_gestalt_rust::Output<String>,
        pub git_file_sources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerGitFileSource>,
        >,
        pub githubs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerGithub>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ignored_files: pulumi_gestalt_rust::Output<Vec<String>>,
        pub include_build_logs: pulumi_gestalt_rust::Output<String>,
        pub included_files: pulumi_gestalt_rust::Output<Vec<String>>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pubsub_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerPubsubConfig>,
        >,
        pub repository_event_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerRepositoryEventConfig>,
        >,
        pub service_account: pulumi_gestalt_rust::Output<String>,
        pub source_to_builds: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerSourceToBuild>,
        >,
        pub substitutions: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub tags: pulumi_gestalt_rust::Output<Vec<String>>,
        pub trigger_id: pulumi_gestalt_rust::Output<String>,
        pub trigger_templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerTriggerTemplate>,
        >,
        pub webhook_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerWebhookConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetTriggerArgs,
    ) -> GetTriggerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let trigger_id_binding = args.trigger_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudbuild/getTrigger:getTrigger".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "triggerId".into(),
                    value: &trigger_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTriggerResult {
            approval_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("approvalConfigs"),
            ),
            bitbucket_server_trigger_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bitbucketServerTriggerConfigs"),
            ),
            builds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("builds"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            filename: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filename"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            git_file_sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gitFileSources"),
            ),
            githubs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("githubs"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ignored_files: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ignoredFiles"),
            ),
            include_build_logs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includeBuildLogs"),
            ),
            included_files: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includedFiles"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pubsub_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pubsubConfigs"),
            ),
            repository_event_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryEventConfigs"),
            ),
            service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccount"),
            ),
            source_to_builds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceToBuilds"),
            ),
            substitutions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("substitutions"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            trigger_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerId"),
            ),
            trigger_templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerTemplates"),
            ),
            webhook_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("webhookConfigs"),
            ),
        }
    }
}
