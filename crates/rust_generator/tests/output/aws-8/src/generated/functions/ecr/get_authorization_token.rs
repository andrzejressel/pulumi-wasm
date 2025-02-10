#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_authorization_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizationTokenArgs {
        /// AWS account ID of the ECR Repository. If not specified the default account is assumed.
        #[builder(into, default)]
        pub registry_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizationTokenResult {
        /// Temporary IAM authentication credentials to access the ECR repository encoded in base64 in the form of `user_name:password`.
        pub authorization_token: pulumi_gestalt_rust::Output<String>,
        /// Time in UTC RFC3339 format when the authorization token expires.
        pub expires_at: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Password decoded from the authorization token.
        pub password: pulumi_gestalt_rust::Output<String>,
        /// Registry URL to use in the docker login command.
        pub proxy_endpoint: pulumi_gestalt_rust::Output<String>,
        pub registry_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// User name decoded from the authorization token.
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAuthorizationTokenArgs,
    ) -> GetAuthorizationTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let registry_id_binding = args.registry_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecr/getAuthorizationToken:getAuthorizationToken".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryId".into(),
                    value: registry_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAuthorizationTokenResult {
            authorization_token: o.get_field("authorizationToken"),
            expires_at: o.get_field("expiresAt"),
            id: o.get_field("id"),
            password: o.get_field("password"),
            proxy_endpoint: o.get_field("proxyEndpoint"),
            registry_id: o.get_field("registryId"),
            user_name: o.get_field("userName"),
        }
    }
}
