#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_pull_through_cache_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPullThroughCacheRuleArgs {
        /// The repository name prefix to use when caching images from the source registry.
        #[builder(into)]
        pub ecr_repository_prefix: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPullThroughCacheRuleResult {
        /// ARN of the Secret which will be used to authenticate against the registry.
        pub credential_arn: pulumi_gestalt_rust::Output<String>,
        pub ecr_repository_prefix: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// The registry URL of the upstream public registry to use as the source.
        pub upstream_registry_url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPullThroughCacheRuleArgs,
    ) -> GetPullThroughCacheRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ecr_repository_prefix_binding = args
            .ecr_repository_prefix
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecr/getPullThroughCacheRule:getPullThroughCacheRule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ecrRepositoryPrefix".into(),
                    value: ecr_repository_prefix_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPullThroughCacheRuleResult {
            credential_arn: o.get_field("credentialArn"),
            ecr_repository_prefix: o.get_field("ecrRepositoryPrefix"),
            id: o.get_field("id"),
            registry_id: o.get_field("registryId"),
            upstream_registry_url: o.get_field("upstreamRegistryUrl"),
        }
    }
}
