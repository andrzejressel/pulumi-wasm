#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_identity_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIdentityPoolArgs {
        /// The Cognito Identity Pool name.
        #[builder(into)]
        pub identity_pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assigned to the Identity Pool.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetIdentityPoolResult {
        /// Whether the classic / basic authentication flow is enabled.
        pub allow_classic_flow: pulumi_gestalt_rust::Output<bool>,
        /// Whether the identity pool supports unauthenticated logins or not.
        pub allow_unauthenticated_identities: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the Pool.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// An array of Amazon Cognito Identity user pools and their client IDs.
        pub cognito_identity_providers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cognito::GetIdentityPoolCognitoIdentityProvider,
            >,
        >,
        /// The "domain" by which Cognito will refer to your users.
        pub developer_provider_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identity_pool_name: pulumi_gestalt_rust::Output<String>,
        /// Set of OpendID Connect provider ARNs.
        pub openid_connect_provider_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
        pub saml_provider_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Key-Value pairs mapping provider names to provider app IDs.
        pub supported_login_providers: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A map of tags to assigned to the Identity Pool.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetIdentityPoolArgs,
    ) -> GetIdentityPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_pool_name_binding = args.identity_pool_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cognito/getIdentityPool:getIdentityPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityPoolName".into(),
                    value: identity_pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetIdentityPoolResult {
            allow_classic_flow: o.get_field("allowClassicFlow"),
            allow_unauthenticated_identities: o
                .get_field("allowUnauthenticatedIdentities"),
            arn: o.get_field("arn"),
            cognito_identity_providers: o.get_field("cognitoIdentityProviders"),
            developer_provider_name: o.get_field("developerProviderName"),
            id: o.get_field("id"),
            identity_pool_name: o.get_field("identityPoolName"),
            openid_connect_provider_arns: o.get_field("openidConnectProviderArns"),
            saml_provider_arns: o.get_field("samlProviderArns"),
            supported_login_providers: o.get_field("supportedLoginProviders"),
            tags: o.get_field("tags"),
        }
    }
}
