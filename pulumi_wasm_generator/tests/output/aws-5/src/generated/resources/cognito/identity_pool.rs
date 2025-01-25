/// Provides an AWS Cognito Identity Pool.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: aws:iam:SamlProvider
///     properties:
///       name: my-saml-provider
///       samlMetadataDocument:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: saml-metadata.xml
///           return: result
///   main:
///     type: aws:cognito:IdentityPool
///     properties:
///       identityPoolName: identity pool
///       allowUnauthenticatedIdentities: false
///       allowClassicFlow: false
///       cognitoIdentityProviders:
///         - clientId: 6lhlkkfbfb4q5kpp90urffae
///           providerName: cognito-idp.us-east-1.amazonaws.com/us-east-1_Tv0493apJ
///           serverSideTokenCheck: false
///         - clientId: 7kodkvfqfb4qfkp39eurffae
///           providerName: cognito-idp.us-east-1.amazonaws.com/eu-west-1_Zr231apJu
///           serverSideTokenCheck: false
///       supportedLoginProviders:
///         graph.facebook.com: '7346241598935552'
///         accounts.google.com: 123456789012.apps.googleusercontent.com
///       samlProviderArns:
///         - ${default.arn}
///       openidConnectProviderArns:
///         - arn:aws:iam::123456789012:oidc-provider/id.example.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito Identity Pool using its ID. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityPool:IdentityPool mypool us-west-2:1a234567-8901-234b-5cde-f6789g01h2i3
/// ```
pub mod identity_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolArgs {
        /// Enables or disables the classic / basic authentication flow. Default is `false`.
        #[builder(into, default)]
        pub allow_classic_flow: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether the identity pool supports unauthenticated logins or not.
        #[builder(into, default)]
        pub allow_unauthenticated_identities: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An array of Amazon Cognito Identity user pools and their client IDs.
        #[builder(into, default)]
        pub cognito_identity_providers: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::cognito::IdentityPoolCognitoIdentityProvider>,
            >,
        >,
        /// The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your
        /// backend and the Cognito service to communicate about the developer provider.
        #[builder(into, default)]
        pub developer_provider_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Cognito Identity Pool name.
        #[builder(into)]
        pub identity_pool_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Set of OpendID Connect provider ARNs.
        #[builder(into, default)]
        pub openid_connect_provider_arns: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// An array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
        #[builder(into, default)]
        pub saml_provider_arns: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-Value pairs mapping provider names to provider app IDs.
        #[builder(into, default)]
        pub supported_login_providers: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags to assign to the Identity Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IdentityPoolResult {
        /// Enables or disables the classic / basic authentication flow. Default is `false`.
        pub allow_classic_flow: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the identity pool supports unauthenticated logins or not.
        pub allow_unauthenticated_identities: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the identity pool.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// An array of Amazon Cognito Identity user pools and their client IDs.
        pub cognito_identity_providers: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cognito::IdentityPoolCognitoIdentityProvider>,
            >,
        >,
        /// The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your
        /// backend and the Cognito service to communicate about the developer provider.
        pub developer_provider_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Cognito Identity Pool name.
        pub identity_pool_name: pulumi_wasm_rust::Output<String>,
        /// Set of OpendID Connect provider ARNs.
        pub openid_connect_provider_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
        pub saml_provider_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Key-Value pairs mapping provider names to provider app IDs.
        pub supported_login_providers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags to assign to the Identity Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IdentityPoolArgs,
    ) -> IdentityPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_classic_flow_binding = args
            .allow_classic_flow
            .get_output(context)
            .get_inner();
        let allow_unauthenticated_identities_binding = args
            .allow_unauthenticated_identities
            .get_output(context)
            .get_inner();
        let cognito_identity_providers_binding = args
            .cognito_identity_providers
            .get_output(context)
            .get_inner();
        let developer_provider_name_binding = args
            .developer_provider_name
            .get_output(context)
            .get_inner();
        let identity_pool_name_binding = args
            .identity_pool_name
            .get_output(context)
            .get_inner();
        let openid_connect_provider_arns_binding = args
            .openid_connect_provider_arns
            .get_output(context)
            .get_inner();
        let saml_provider_arns_binding = args
            .saml_provider_arns
            .get_output(context)
            .get_inner();
        let supported_login_providers_binding = args
            .supported_login_providers
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/identityPool:IdentityPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowClassicFlow".into(),
                    value: &allow_classic_flow_binding,
                },
                register_interface::ObjectField {
                    name: "allowUnauthenticatedIdentities".into(),
                    value: &allow_unauthenticated_identities_binding,
                },
                register_interface::ObjectField {
                    name: "cognitoIdentityProviders".into(),
                    value: &cognito_identity_providers_binding,
                },
                register_interface::ObjectField {
                    name: "developerProviderName".into(),
                    value: &developer_provider_name_binding,
                },
                register_interface::ObjectField {
                    name: "identityPoolName".into(),
                    value: &identity_pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "openidConnectProviderArns".into(),
                    value: &openid_connect_provider_arns_binding,
                },
                register_interface::ObjectField {
                    name: "samlProviderArns".into(),
                    value: &saml_provider_arns_binding,
                },
                register_interface::ObjectField {
                    name: "supportedLoginProviders".into(),
                    value: &supported_login_providers_binding,
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
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityPoolResult {
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
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
