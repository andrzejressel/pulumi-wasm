#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_repository_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryEndpointArgs {
        /// Name of the domain that contains the repository.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Account number of the AWS account that owns the domain.
        #[builder(into, default)]
        pub domain_owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Which endpoint of a repository to return. A repository has one endpoint for each package format: `npm`, `pypi`, `maven`, and `nuget`.
        #[builder(into)]
        pub format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the repository.
        #[builder(into)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryEndpointResult {
        pub domain: pulumi_gestalt_rust::Output<String>,
        pub domain_owner: pulumi_gestalt_rust::Output<String>,
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub repository: pulumi_gestalt_rust::Output<String>,
        /// URL of the returned endpoint.
        pub repository_endpoint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRepositoryEndpointArgs,
    ) -> GetRepositoryEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let domain_owner_binding = args.domain_owner.get_output(context);
        let format_binding = args.format.get_output(context);
        let repository_binding = args.repository.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:codeartifact/getRepositoryEndpoint:getRepositoryEndpoint".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainOwner".into(),
                    value: &domain_owner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: &format_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repository".into(),
                    value: &repository_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRepositoryEndpointResult {
            domain: o.get_field("domain"),
            domain_owner: o.get_field("domainOwner"),
            format: o.get_field("format"),
            id: o.get_field("id"),
            repository: o.get_field("repository"),
            repository_endpoint: o.get_field("repositoryEndpoint"),
        }
    }
}
