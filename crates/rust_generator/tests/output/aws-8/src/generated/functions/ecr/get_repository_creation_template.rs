#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_repository_creation_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryCreationTemplateArgs {
        /// The repository name prefix that the template matches against.
        #[builder(into)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to any created repositories.
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryCreationTemplateResult {
        /// Which features this template applies to. Contains one or more of `PULL_THROUGH_CACHE` or `REPLICATION`.
        pub applied_fors: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ARN of the custom role used for repository creation.
        pub custom_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The description for this template.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Encryption configuration for any created repositories. See Encryption Configuration below.
        pub encryption_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ecr::GetRepositoryCreationTemplateEncryptionConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The tag mutability setting for any created repositories.
        pub image_tag_mutability: pulumi_gestalt_rust::Output<String>,
        /// The lifecycle policy document to apply to any created repositories.
        pub lifecycle_policy: pulumi_gestalt_rust::Output<String>,
        pub prefix: pulumi_gestalt_rust::Output<String>,
        /// The registry ID the repository creation template applies to.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// The registry policy document to apply to any created repositories.
        pub repository_policy: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to any created repositories.
        pub resource_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRepositoryCreationTemplateArgs,
    ) -> GetRepositoryCreationTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let prefix_binding_1 = args.prefix.get_output(context);
        let prefix_binding = prefix_binding_1.get_inner();
        let resource_tags_binding_1 = args.resource_tags.get_output(context);
        let resource_tags_binding = resource_tags_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRepositoryCreationTemplateResult {
            applied_fors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appliedFors"),
            ),
            custom_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customRoleArn"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encryption_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfigurations"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            image_tag_mutability: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageTagMutability"),
            ),
            lifecycle_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lifecyclePolicy"),
            ),
            prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefix"),
            ),
            registry_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            repository_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryPolicy"),
            ),
            resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTags"),
            ),
        }
    }
}
