/// Provides a CodeArtifact Repository Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryArgs {
        /// The description of the repository.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The domain that contains the created repository.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// An array of external connections associated with the repository. Only one external connection can be set per repository. see External Connections.
        #[builder(into, default)]
        pub external_connections: pulumi_wasm_rust::Output<
            Option<super::super::types::codeartifact::RepositoryExternalConnections>,
        >,
        /// The name of the repository to create.
        #[builder(into)]
        pub repository: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of upstream repositories to associate with the repository. The order of the upstream repositories in the list determines their priority order when AWS CodeArtifact looks for a requested package version. see Upstream
        #[builder(into, default)]
        pub upstreams: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codeartifact::RepositoryUpstream>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RepositoryResult {
        /// The account number of the AWS account that manages the repository.
        pub administrator_account: pulumi_wasm_rust::Output<String>,
        /// The ARN of the repository.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The description of the repository.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The domain that contains the created repository.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The account number of the AWS account that owns the domain.
        pub domain_owner: pulumi_wasm_rust::Output<String>,
        /// An array of external connections associated with the repository. Only one external connection can be set per repository. see External Connections.
        pub external_connections: pulumi_wasm_rust::Output<
            Option<super::super::types::codeartifact::RepositoryExternalConnections>,
        >,
        /// The name of the repository to create.
        pub repository: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of upstream repositories to associate with the repository. The order of the upstream repositories in the list determines their priority order when AWS CodeArtifact looks for a requested package version. see Upstream
        pub upstreams: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codeartifact::RepositoryUpstream>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RepositoryArgs) -> RepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let domain_binding = args.domain.get_inner();
        let domain_owner_binding = args.domain_owner.get_inner();
        let external_connections_binding = args.external_connections.get_inner();
        let repository_binding = args.repository.get_inner();
        let tags_binding = args.tags.get_inner();
        let upstreams_binding = args.upstreams.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codeartifact/repository:Repository".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorAccount".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "domainOwner".into(),
                },
                register_interface::ResultField {
                    name: "externalConnections".into(),
                },
                register_interface::ResultField {
                    name: "repository".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "upstreams".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RepositoryResult {
            administrator_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorAccount").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            domain_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainOwner").unwrap(),
            ),
            external_connections: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalConnections").unwrap(),
            ),
            repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repository").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            upstreams: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upstreams").unwrap(),
            ),
        }
    }
}