pub mod get_trigger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTriggerArgs {
        /// The Cloud Build location for the trigger.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier for the trigger..
        #[builder(into)]
        pub trigger_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTriggerResult {
        pub approval_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerApprovalConfig>,
        >,
        pub bitbucket_server_trigger_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudbuild::GetTriggerBitbucketServerTriggerConfig,
            >,
        >,
        pub builds: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerBuild>,
        >,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub disabled: pulumi_wasm_rust::Output<bool>,
        pub filename: pulumi_wasm_rust::Output<String>,
        pub filter: pulumi_wasm_rust::Output<String>,
        pub git_file_sources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerGitFileSource>,
        >,
        pub githubs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerGithub>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ignored_files: pulumi_wasm_rust::Output<Vec<String>>,
        pub include_build_logs: pulumi_wasm_rust::Output<String>,
        pub included_files: pulumi_wasm_rust::Output<Vec<String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pubsub_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerPubsubConfig>,
        >,
        pub repository_event_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerRepositoryEventConfig>,
        >,
        pub service_account: pulumi_wasm_rust::Output<String>,
        pub source_to_builds: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerSourceToBuild>,
        >,
        pub substitutions: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        pub trigger_id: pulumi_wasm_rust::Output<String>,
        pub trigger_templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerTriggerTemplate>,
        >,
        pub webhook_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudbuild::GetTriggerWebhookConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTriggerArgs) -> GetTriggerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let trigger_id_binding = args.trigger_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudbuild/getTrigger:getTrigger".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "approvalConfigs".into(),
                },
                register_interface::ResultField {
                    name: "bitbucketServerTriggerConfigs".into(),
                },
                register_interface::ResultField {
                    name: "builds".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "filename".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "gitFileSources".into(),
                },
                register_interface::ResultField {
                    name: "githubs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ignoredFiles".into(),
                },
                register_interface::ResultField {
                    name: "includeBuildLogs".into(),
                },
                register_interface::ResultField {
                    name: "includedFiles".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pubsubConfigs".into(),
                },
                register_interface::ResultField {
                    name: "repositoryEventConfigs".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "sourceToBuilds".into(),
                },
                register_interface::ResultField {
                    name: "substitutions".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "triggerId".into(),
                },
                register_interface::ResultField {
                    name: "triggerTemplates".into(),
                },
                register_interface::ResultField {
                    name: "webhookConfigs".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTriggerResult {
            approval_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalConfigs").unwrap(),
            ),
            bitbucket_server_trigger_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bitbucketServerTriggerConfigs").unwrap(),
            ),
            builds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("builds").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            filename: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filename").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            git_file_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gitFileSources").unwrap(),
            ),
            githubs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("githubs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ignored_files: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoredFiles").unwrap(),
            ),
            include_build_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeBuildLogs").unwrap(),
            ),
            included_files: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includedFiles").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pubsub_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pubsubConfigs").unwrap(),
            ),
            repository_event_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryEventConfigs").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            source_to_builds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceToBuilds").unwrap(),
            ),
            substitutions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("substitutions").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            trigger_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerId").unwrap(),
            ),
            trigger_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerTemplates").unwrap(),
            ),
            webhook_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webhookConfigs").unwrap(),
            ),
        }
    }
}
