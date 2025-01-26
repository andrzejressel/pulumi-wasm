/// Provides a Cognito User Identity Provider resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cognito:UserPool
///     properties:
///       name: example-pool
///       autoVerifiedAttributes:
///         - email
///   exampleProvider:
///     type: aws:cognito:IdentityProvider
///     name: example_provider
///     properties:
///       userPoolId: ${example.id}
///       providerName: Google
///       providerType: Google
///       providerDetails:
///         authorize_scopes: email
///         client_id: your client_id
///         client_secret: your client_secret
///       attributeMapping:
///         email: email
///         username: sub
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_cognito_identity_provider` resources using their User Pool ID and Provider Name. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityProvider:IdentityProvider example us-west-2_abc123:CorpAD
/// ```
pub mod identity_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityProviderArgs {
        /// The map of attribute mapping of user pool attributes. [AttributeMapping in AWS API documentation](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-AttributeMapping)
        #[builder(into, default)]
        pub attribute_mapping: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The list of identity providers.
        #[builder(into, default)]
        pub idp_identifiers: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The map of identity details, such as access token
        #[builder(into)]
        pub provider_details: pulumi_wasm_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// The provider name
        #[builder(into)]
        pub provider_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The provider type.  [See AWS API for valid values](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-ProviderType)
        #[builder(into)]
        pub provider_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The user pool id
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IdentityProviderResult {
        /// The map of attribute mapping of user pool attributes. [AttributeMapping in AWS API documentation](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-AttributeMapping)
        pub attribute_mapping: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The list of identity providers.
        pub idp_identifiers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The map of identity details, such as access token
        pub provider_details: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The provider name
        pub provider_name: pulumi_wasm_rust::Output<String>,
        /// The provider type.  [See AWS API for valid values](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#CognitoUserPools-CreateIdentityProvider-request-ProviderType)
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// The user pool id
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IdentityProviderArgs,
    ) -> IdentityProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attribute_mapping_binding = args
            .attribute_mapping
            .get_output(context)
            .get_inner();
        let idp_identifiers_binding = args
            .idp_identifiers
            .get_output(context)
            .get_inner();
        let provider_details_binding = args
            .provider_details
            .get_output(context)
            .get_inner();
        let provider_name_binding = args.provider_name.get_output(context).get_inner();
        let provider_type_binding = args.provider_type.get_output(context).get_inner();
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/identityProvider:IdentityProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributeMapping".into(),
                    value: &attribute_mapping_binding,
                },
                register_interface::ObjectField {
                    name: "idpIdentifiers".into(),
                    value: &idp_identifiers_binding,
                },
                register_interface::ObjectField {
                    name: "providerDetails".into(),
                    value: &provider_details_binding,
                },
                register_interface::ObjectField {
                    name: "providerName".into(),
                    value: &provider_name_binding,
                },
                register_interface::ObjectField {
                    name: "providerType".into(),
                    value: &provider_type_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attributeMapping".into(),
                },
                register_interface::ResultField {
                    name: "idpIdentifiers".into(),
                },
                register_interface::ResultField {
                    name: "providerDetails".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
                register_interface::ResultField {
                    name: "providerType".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityProviderResult {
            attribute_mapping: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributeMapping").unwrap(),
            ),
            idp_identifiers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("idpIdentifiers").unwrap(),
            ),
            provider_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerDetails").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
            provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerType").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
        }
    }
}
