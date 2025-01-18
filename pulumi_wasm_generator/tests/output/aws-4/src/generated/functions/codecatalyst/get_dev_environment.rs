pub mod get_dev_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDevEnvironmentArgs {
        /// The user-specified alias for the Dev Environment.
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// The system-generated unique ID of the user who created the Dev Environment.
        #[builder(into, default)]
        pub creator_id: pulumi_wasm_rust::Output<Option<String>>,
        /// - (Required) The system-generated unique ID of the Dev Environment for which you want to view information. To retrieve a list of Dev Environment IDs, use [ListDevEnvironments](https://docs.aws.amazon.com/codecatalyst/latest/APIReference/API_ListDevEnvironments.html).
        #[builder(into)]
        pub env_id: pulumi_wasm_rust::Output<String>,
        /// The name of the project in the space.
        #[builder(into)]
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        #[builder(into, default)]
        pub repositories: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::codecatalyst::GetDevEnvironmentRepository,
                >,
            >,
        >,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDevEnvironmentResult {
        /// The user-specified alias for the Dev Environment.
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// The system-generated unique ID of the user who created the Dev Environment.
        pub creator_id: pulumi_wasm_rust::Output<Option<String>>,
        pub env_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Information about the integrated development environment (IDE) configured for a Dev Environment.
        pub ides: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::codecatalyst::GetDevEnvironmentIde>,
        >,
        /// The amount of time the Dev Environment will run without any activity detected before stopping, in minutes. Only whole integers are allowed. Dev Environments consume compute minutes when running.
        pub inactivity_timeout_minutes: pulumi_wasm_rust::Output<i32>,
        /// The Amazon EC2 instace type to use for the Dev Environment.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// The time when the Dev Environment was last updated, in coordinated universal time (UTC) timestamp format as specified in [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339#section-5.6).
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// Information about the amount of storage allocated to the Dev Environment.
        pub persistent_storages: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::codecatalyst::GetDevEnvironmentPersistentStorage,
            >,
        >,
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        pub repositories: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::codecatalyst::GetDevEnvironmentRepository,
                >,
            >,
        >,
        pub space_name: pulumi_wasm_rust::Output<String>,
        /// The current status of the Dev Environment. From: PENDING | RUNNING | STARTING | STOPPING | STOPPED | FAILED | DELETING | DELETED.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The reason for the status.
        pub status_reason: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDevEnvironmentArgs) -> GetDevEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let creator_id_binding = args.creator_id.get_inner();
        let env_id_binding = args.env_id.get_inner();
        let project_name_binding = args.project_name.get_inner();
        let repositories_binding = args.repositories.get_inner();
        let space_name_binding = args.space_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codecatalyst/getDevEnvironment:getDevEnvironment".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "creatorId".into(),
                    value: &creator_id_binding,
                },
                register_interface::ObjectField {
                    name: "envId".into(),
                    value: &env_id_binding,
                },
                register_interface::ObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding,
                },
                register_interface::ObjectField {
                    name: "repositories".into(),
                    value: &repositories_binding,
                },
                register_interface::ObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "creatorId".into(),
                },
                register_interface::ResultField {
                    name: "envId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ides".into(),
                },
                register_interface::ResultField {
                    name: "inactivityTimeoutMinutes".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTime".into(),
                },
                register_interface::ResultField {
                    name: "persistentStorages".into(),
                },
                register_interface::ResultField {
                    name: "projectName".into(),
                },
                register_interface::ResultField {
                    name: "repositories".into(),
                },
                register_interface::ResultField {
                    name: "spaceName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusReason".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDevEnvironmentResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            creator_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creatorId").unwrap(),
            ),
            env_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ides: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ides").unwrap(),
            ),
            inactivity_timeout_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inactivityTimeoutMinutes").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTime").unwrap(),
            ),
            persistent_storages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persistentStorages").unwrap(),
            ),
            project_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectName").unwrap(),
            ),
            repositories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositories").unwrap(),
            ),
            space_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusReason").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
