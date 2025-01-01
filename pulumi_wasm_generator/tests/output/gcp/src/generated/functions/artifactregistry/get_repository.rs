pub mod get_repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryArgs {
        /// The location of the artifact registry repository. eg us-central1
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The last part of the repository name.
        #[builder(into)]
        pub repository_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryResult {
        pub cleanup_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::artifactregistry::GetRepositoryCleanupPolicy>,
        >,
        pub cleanup_policy_dry_run: pulumi_wasm_rust::Output<bool>,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub docker_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::artifactregistry::GetRepositoryDockerConfig>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub format: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub maven_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::artifactregistry::GetRepositoryMavenConfig>,
        >,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub remote_repository_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfig,
            >,
        >,
        pub repository_id: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
        pub virtual_repository_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::artifactregistry::GetRepositoryVirtualRepositoryConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRepositoryArgs) -> GetRepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let repository_id_binding = args.repository_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:artifactregistry/getRepository:getRepository".into(),
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
                    name: "repositoryId".into(),
                    value: &repository_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cleanupPolicies".into(),
                },
                register_interface::ResultField {
                    name: "cleanupPolicyDryRun".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dockerConfigs".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "format".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mavenConfigs".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "remoteRepositoryConfigs".into(),
                },
                register_interface::ResultField {
                    name: "repositoryId".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "virtualRepositoryConfigs".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRepositoryResult {
            cleanup_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cleanupPolicies").unwrap(),
            ),
            cleanup_policy_dry_run: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cleanupPolicyDryRun").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            docker_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dockerConfigs").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("format").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maven_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mavenConfigs").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            remote_repository_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteRepositoryConfigs").unwrap(),
            ),
            repository_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryId").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            virtual_repository_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualRepositoryConfigs").unwrap(),
            ),
        }
    }
}
