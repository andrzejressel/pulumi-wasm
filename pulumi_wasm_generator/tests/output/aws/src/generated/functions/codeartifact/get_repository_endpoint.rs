pub mod get_repository_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryEndpointArgs {
        /// Name of the domain that contains the repository.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Which endpoint of a repository to return. A repository has one endpoint for each package format: `npm`, `pypi`, `maven`, and `nuget`.
        #[builder(into)]
        pub format: pulumi_wasm_rust::Output<String>,
        /// Name of the repository.
        #[builder(into)]
        pub repository: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryEndpointResult {
        pub domain: pulumi_wasm_rust::Output<String>,
        pub domain_owner: pulumi_wasm_rust::Output<String>,
        pub format: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub repository: pulumi_wasm_rust::Output<String>,
        /// URL of the returned endpoint.
        pub repository_endpoint: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRepositoryEndpointArgs) -> GetRepositoryEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_inner();
        let domain_owner_binding = args.domain_owner.get_inner();
        let format_binding = args.format.get_inner();
        let repository_binding = args.repository.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codeartifact/getRepositoryEndpoint:getRepositoryEndpoint".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "domainOwner".into(),
                    value: &domain_owner_binding,
                },
                register_interface::ObjectField {
                    name: "format".into(),
                    value: &format_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "domainOwner".into(),
                },
                register_interface::ResultField {
                    name: "format".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "repository".into(),
                },
                register_interface::ResultField {
                    name: "repositoryEndpoint".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRepositoryEndpointResult {
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            domain_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainOwner").unwrap(),
            ),
            format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("format").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repository").unwrap(),
            ),
            repository_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryEndpoint").unwrap(),
            ),
        }
    }
}