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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetIdentityPoolArgs,
    ) -> GetIdentityPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let identity_pool_name_binding = args
            .identity_pool_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getIdentityPool:getIdentityPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identityPoolName".into(),
                    value: &identity_pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIdentityPoolResult {
            allow_classic_flow: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowClassicFlow"),
            ),
            allow_unauthenticated_identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowUnauthenticatedIdentities"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cognito_identity_providers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cognitoIdentityProviders"),
            ),
            developer_provider_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("developerProviderName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identity_pool_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityPoolName"),
            ),
            openid_connect_provider_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("openidConnectProviderArns"),
            ),
            saml_provider_arns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("samlProviderArns"),
            ),
            supported_login_providers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("supportedLoginProviders"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
