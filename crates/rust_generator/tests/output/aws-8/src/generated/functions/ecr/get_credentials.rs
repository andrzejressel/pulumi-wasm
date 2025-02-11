#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_credentials {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCredentialsArgs {
        #[builder(into)]
        pub registry_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCredentialsResult {
        pub authorization_token: pulumi_gestalt_rust::Output<String>,
        pub expires_at: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub proxy_endpoint: pulumi_gestalt_rust::Output<String>,
        pub registry_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCredentialsArgs,
    ) -> GetCredentialsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let registry_id_binding = args.registry_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecr/getCredentials:getCredentials".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryId".into(),
                    value: &registry_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCredentialsResult {
            authorization_token: o.get_field("authorizationToken"),
            expires_at: o.get_field("expiresAt"),
            id: o.get_field("id"),
            proxy_endpoint: o.get_field("proxyEndpoint"),
            registry_id: o.get_field("registryId"),
        }
    }
}
