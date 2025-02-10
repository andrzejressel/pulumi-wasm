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
        context: &pulumi_gestalt_rust::Context,
        args: GetTriggerArgs,
    ) -> GetTriggerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let trigger_id_binding = args.trigger_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudbuild/getTrigger:getTrigger".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerId".into(),
                    value: trigger_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTriggerResult {
            approval_configs: o.get_field("approvalConfigs"),
            bitbucket_server_trigger_configs: o
                .get_field("bitbucketServerTriggerConfigs"),
            builds: o.get_field("builds"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            filename: o.get_field("filename"),
            filter: o.get_field("filter"),
            git_file_sources: o.get_field("gitFileSources"),
            githubs: o.get_field("githubs"),
            id: o.get_field("id"),
            ignored_files: o.get_field("ignoredFiles"),
            include_build_logs: o.get_field("includeBuildLogs"),
            included_files: o.get_field("includedFiles"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pubsub_configs: o.get_field("pubsubConfigs"),
            repository_event_configs: o.get_field("repositoryEventConfigs"),
            service_account: o.get_field("serviceAccount"),
            source_to_builds: o.get_field("sourceToBuilds"),
            substitutions: o.get_field("substitutions"),
            tags: o.get_field("tags"),
            trigger_id: o.get_field("triggerId"),
            trigger_templates: o.get_field("triggerTemplates"),
            webhook_configs: o.get_field("webhookConfigs"),
        }
    }
}
