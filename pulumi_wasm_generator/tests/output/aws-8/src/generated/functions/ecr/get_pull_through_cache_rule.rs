pub mod get_pull_through_cache_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPullThroughCacheRuleArgs {
        /// The repository name prefix to use when caching images from the source registry.
        #[builder(into)]
        pub ecr_repository_prefix: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPullThroughCacheRuleResult {
        /// ARN of the Secret which will be used to authenticate against the registry.
        pub credential_arn: pulumi_wasm_rust::Output<String>,
        pub ecr_repository_prefix: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// The registry URL of the upstream public registry to use as the source.
        pub upstream_registry_url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPullThroughCacheRuleArgs) -> GetPullThroughCacheRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ecr_repository_prefix_binding = args.ecr_repository_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getPullThroughCacheRule:getPullThroughCacheRule".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ecrRepositoryPrefix".into(),
                    value: &ecr_repository_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "credentialArn".into(),
                },
                register_interface::ResultField {
                    name: "ecrRepositoryPrefix".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "upstreamRegistryUrl".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPullThroughCacheRuleResult {
            credential_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialArn").unwrap(),
            ),
            ecr_repository_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ecrRepositoryPrefix").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            upstream_registry_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upstreamRegistryUrl").unwrap(),
            ),
        }
    }
}
