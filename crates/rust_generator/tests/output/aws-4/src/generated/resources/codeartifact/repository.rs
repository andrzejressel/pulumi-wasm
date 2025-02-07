/// Provides a CodeArtifact Repository Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder().description("domain key").build_struct(),
///     );
///     let exampleDomain = domain::create(
///         "exampleDomain",
///         DomainArgs::builder()
///             .domain("example")
///             .encryption_key("${example.arn}")
///             .build_struct(),
///     );
///     let test = repository::create(
///         "test",
///         RepositoryArgs::builder()
///             .domain("${exampleDomain.domain}")
///             .repository("example")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Upstream Repository
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = repository::create(
///         "test",
///         RepositoryArgs::builder()
///             .domain("${example.domain}")
///             .repository("example")
///             .upstreams(
///                 vec![
///                     RepositoryUpstream::builder()
///                     .repositoryName("${upstream.repository}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let upstream = repository::create(
///         "upstream",
///         RepositoryArgs::builder()
///             .domain("${testAwsCodeartifactDomain.domain}")
///             .repository("upstream")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With External Connection
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = repository::create(
///         "test",
///         RepositoryArgs::builder()
///             .domain("${example.domain}")
///             .external_connections(
///                 RepositoryExternalConnections::builder()
///                     .externalConnectionName("public:npmjs")
///                     .build_struct(),
///             )
///             .repository("example")
///             .build_struct(),
///     );
///     let upstream = repository::create(
///         "upstream",
///         RepositoryArgs::builder()
///             .domain("${testAwsCodeartifactDomain.domain}")
///             .repository("upstream")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeArtifact Repository using the CodeArtifact Repository ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codeartifact/repository:Repository example arn:aws:codeartifact:us-west-2:012345678912:repository/tf-acc-test-6968272603913957763/tf-acc-test-6968272603913957763
/// ```
pub mod repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// The description of the repository.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The domain that contains the created repository.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array of external connections associated with the repository. Only one external connection can be set per repository. see External Connections.
        #[builder(into, default)]
        pub external_connections: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codeartifact::RepositoryExternalConnections>,
        >,
        /// The name of the repository to create.
        #[builder(into)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of upstream repositories to associate with the repository. The order of the upstream repositories in the list determines their priority order when AWS CodeArtifact looks for a requested package version. see Upstream
        #[builder(into, default)]
        pub upstreams: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codeartifact::RepositoryUpstream>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// The account number of the AWS account that manages the repository.
        pub administrator_account: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the repository.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the repository.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The domain that contains the created repository.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        pub domain_owner: pulumi_gestalt_rust::Output<String>,
        /// An array of external connections associated with the repository. Only one external connection can be set per repository. see External Connections.
        pub external_connections: pulumi_gestalt_rust::Output<
            Option<super::super::types::codeartifact::RepositoryExternalConnections>,
        >,
        /// The name of the repository to create.
        pub repository: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of upstream repositories to associate with the repository. The order of the upstream repositories in the list determines their priority order when AWS CodeArtifact looks for a requested package version. see Upstream
        pub upstreams: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codeartifact::RepositoryUpstream>>,
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
        let description_binding = args.description.get_output(context).get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let domain_owner_binding = args.domain_owner.get_output(context).get_inner();
        let external_connections_binding = args
            .external_connections
            .get_output(context)
            .get_inner();
        let repository_binding = args.repository.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let upstreams_binding = args.upstreams.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codeartifact/repository:Repository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "domainOwner".into(),
                    value: &domain_owner_binding,
                },
                register_interface::ObjectField {
                    name: "externalConnections".into(),
                    value: &external_connections_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "upstreams".into(),
                    value: &upstreams_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RepositoryResult {
            administrator_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("administratorAccount"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domain"),
            ),
            domain_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainOwner"),
            ),
            external_connections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalConnections"),
            ),
            repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repository"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            upstreams: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("upstreams"),
            ),
        }
    }
}
