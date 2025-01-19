pub mod get_repository_creation_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryCreationTemplateArgs {
        /// The repository name prefix that the template matches against.
        #[builder(into)]
        pub prefix: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to any created repositories.
        #[builder(into, default)]
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryCreationTemplateResult {
        /// Which features this template applies to. Contains one or more of `PULL_THROUGH_CACHE` or `REPLICATION`.
        pub applied_fors: pulumi_wasm_rust::Output<Vec<String>>,
        /// The ARN of the custom role used for repository creation.
        pub custom_role_arn: pulumi_wasm_rust::Output<String>,
        /// The description for this template.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Encryption configuration for any created repositories. See Encryption Configuration below.
        pub encryption_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ecr::GetRepositoryCreationTemplateEncryptionConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The tag mutability setting for any created repositories.
        pub image_tag_mutability: pulumi_wasm_rust::Output<String>,
        /// The lifecycle policy document to apply to any created repositories.
        pub lifecycle_policy: pulumi_wasm_rust::Output<String>,
        pub prefix: pulumi_wasm_rust::Output<String>,
        /// The registry ID the repository creation template applies to.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// The registry policy document to apply to any created repositories.
        pub repository_policy: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to any created repositories.
        pub resource_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetRepositoryCreationTemplateArgs,
    ) -> GetRepositoryCreationTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let prefix_binding = args.prefix.get_inner();
        let resource_tags_binding = args.resource_tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getRepositoryCreationTemplate:getRepositoryCreationTemplate"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appliedFors".into(),
                },
                register_interface::ResultField {
                    name: "customRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageTagMutability".into(),
                },
                register_interface::ResultField {
                    name: "lifecyclePolicy".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryPolicy".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRepositoryCreationTemplateResult {
            applied_fors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appliedFors").unwrap(),
            ),
            custom_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRoleArn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfigurations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_tag_mutability: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTagMutability").unwrap(),
            ),
            lifecycle_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecyclePolicy").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryPolicy").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
        }
    }
}
