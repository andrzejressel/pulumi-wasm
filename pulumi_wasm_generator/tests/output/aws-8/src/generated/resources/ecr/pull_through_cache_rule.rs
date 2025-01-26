/// Provides an Elastic Container Registry Pull Through Cache Rule.
///
/// More information about pull through cache rules, including the set of supported
/// upstream repositories, see [Using pull through cache rules](https://docs.aws.amazon.com/AmazonECR/latest/userguide/pull-through-cache.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = pull_through_cache_rule::create(
///         "example",
///         PullThroughCacheRuleArgs::builder()
///             .credential_arn(
///                 "arn:aws:secretsmanager:us-east-1:123456789:secret:ecr-pullthroughcache/ecrpublic",
///             )
///             .ecr_repository_prefix("ecr-public")
///             .upstream_registry_url("public.ecr.aws")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a pull-through cache rule using the `ecr_repository_prefix`. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/pullThroughCacheRule:PullThroughCacheRule example ecr-public
/// ```
pub mod pull_through_cache_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PullThroughCacheRuleArgs {
        /// ARN of the Secret which will be used to authenticate against the registry.
        #[builder(into, default)]
        pub credential_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The repository name prefix to use when caching images from the source registry.
        #[builder(into)]
        pub ecr_repository_prefix: pulumi_wasm_rust::InputOrOutput<String>,
        /// The registry URL of the upstream public registry to use as the source.
        #[builder(into)]
        pub upstream_registry_url: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PullThroughCacheRuleResult {
        /// ARN of the Secret which will be used to authenticate against the registry.
        pub credential_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The repository name prefix to use when caching images from the source registry.
        pub ecr_repository_prefix: pulumi_wasm_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// The registry URL of the upstream public registry to use as the source.
        pub upstream_registry_url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PullThroughCacheRuleArgs,
    ) -> PullThroughCacheRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let credential_arn_binding = args.credential_arn.get_output(context).get_inner();
        let ecr_repository_prefix_binding = args
            .ecr_repository_prefix
            .get_output(context)
            .get_inner();
        let upstream_registry_url_binding = args
            .upstream_registry_url
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/pullThroughCacheRule:PullThroughCacheRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "credentialArn".into(),
                    value: &credential_arn_binding,
                },
                register_interface::ObjectField {
                    name: "ecrRepositoryPrefix".into(),
                    value: &ecr_repository_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "upstreamRegistryUrl".into(),
                    value: &upstream_registry_url_binding,
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
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "upstreamRegistryUrl".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PullThroughCacheRuleResult {
            credential_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialArn").unwrap(),
            ),
            ecr_repository_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ecrRepositoryPrefix").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            upstream_registry_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upstreamRegistryUrl").unwrap(),
            ),
        }
    }
}
