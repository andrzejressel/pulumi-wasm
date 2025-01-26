pub mod get_identity_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIdentityPoolArgs {
        /// The Cognito Identity Pool name.
        #[builder(into)]
        pub identity_pool_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assigned to the Identity Pool.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetIdentityPoolResult {
        /// Whether the classic / basic authentication flow is enabled.
        pub allow_classic_flow: pulumi_wasm_rust::Output<bool>,
        /// Whether the identity pool supports unauthenticated logins or not.
        pub allow_unauthenticated_identities: pulumi_wasm_rust::Output<bool>,
        /// ARN of the Pool.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// An array of Amazon Cognito Identity user pools and their client IDs.
        pub cognito_identity_providers: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cognito::GetIdentityPoolCognitoIdentityProvider,
            >,
        >,
        /// The "domain" by which Cognito will refer to your users.
        pub developer_provider_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identity_pool_name: pulumi_wasm_rust::Output<String>,
        /// Set of OpendID Connect provider ARNs.
        pub openid_connect_provider_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// An array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
        pub saml_provider_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-Value pairs mapping provider names to provider app IDs.
        pub supported_login_providers: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A map of tags to assigned to the Identity Pool.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetIdentityPoolArgs,
    ) -> GetIdentityPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowClassicFlow".into(),
                },
                register_interface::ResultField {
                    name: "allowUnauthenticatedIdentities".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cognitoIdentityProviders".into(),
                },
                register_interface::ResultField {
                    name: "developerProviderName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityPoolName".into(),
                },
                register_interface::ResultField {
                    name: "openidConnectProviderArns".into(),
                },
                register_interface::ResultField {
                    name: "samlProviderArns".into(),
                },
                register_interface::ResultField {
                    name: "supportedLoginProviders".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetIdentityPoolResult {
            allow_classic_flow: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowClassicFlow").unwrap(),
            ),
            allow_unauthenticated_identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowUnauthenticatedIdentities").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cognito_identity_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitoIdentityProviders").unwrap(),
            ),
            developer_provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("developerProviderName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_pool_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityPoolName").unwrap(),
            ),
            openid_connect_provider_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("openidConnectProviderArns").unwrap(),
            ),
            saml_provider_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samlProviderArns").unwrap(),
            ),
            supported_login_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedLoginProviders").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
