/// Provides a Public Elastic Container Registry Repository.
///
/// > **NOTE:** This resource can only be used in the `us-east-1` region.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:ecrpublic:Repository
///     properties:
///       repositoryName: bar
///       catalogData:
///         aboutText: About Text
///         architectures:
///           - ARM
///         description: Description
///         logoImageBlob:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: ${png}
///             return: result
///         operatingSystems:
///           - Linux
///         usageText: Usage Text
///       tags:
///         env: production
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Public Repositories using the `repository_name`. For example:
///
/// ```sh
/// $ pulumi import aws:ecrpublic/repository:Repository example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// Catalog data configuration for the repository. See below for schema.
        #[builder(into, default)]
        pub catalog_data: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ecrpublic::RepositoryCatalogData>,
        >,
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the repository.
        #[builder(into)]
        pub repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// Full ARN of the repository.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Catalog data configuration for the repository. See below for schema.
        pub catalog_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::ecrpublic::RepositoryCatalogData>,
        >,
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the repository.
        pub repository_name: pulumi_gestalt_rust::Output<String>,
        /// The URI of the repository.
        pub repository_uri: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RepositoryArgs,
    ) -> RepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let catalog_data_binding_1 = args.catalog_data.get_output(context);
        let catalog_data_binding = catalog_data_binding_1.get_inner();
        let force_destroy_binding_1 = args.force_destroy.get_output(context);
        let force_destroy_binding = force_destroy_binding_1.get_inner();
        let repository_name_binding_1 = args.repository_name.get_output(context);
        let repository_name_binding = repository_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecrpublic/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogData".into(),
                    value: &catalog_data_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            catalog_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("catalogData"),
            ),
            force_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDestroy"),
            ),
            registry_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            repository_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryName"),
            ),
            repository_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryUri"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
