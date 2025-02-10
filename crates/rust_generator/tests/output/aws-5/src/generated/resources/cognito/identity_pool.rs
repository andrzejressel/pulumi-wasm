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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolArgs {
        /// Enables or disables the classic / basic authentication flow. Default is `false`.
        #[builder(into, default)]
        pub allow_classic_flow: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the identity pool supports unauthenticated logins or not.
        #[builder(into, default)]
        pub allow_unauthenticated_identities: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An array of Amazon Cognito Identity user pools and their client IDs.
        #[builder(into, default)]
        pub cognito_identity_providers: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::cognito::IdentityPoolCognitoIdentityProvider>,
            >,
        >,
        /// The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your
        /// backend and the Cognito service to communicate about the developer provider.
        #[builder(into, default)]
        pub developer_provider_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Cognito Identity Pool name.
        #[builder(into)]
        pub identity_pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set of OpendID Connect provider ARNs.
        #[builder(into, default)]
        pub openid_connect_provider_arns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// An array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
        #[builder(into, default)]
        pub saml_provider_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-Value pairs mapping provider names to provider app IDs.
        #[builder(into, default)]
        pub supported_login_providers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags to assign to the Identity Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IdentityPoolResult {
        /// Enables or disables the classic / basic authentication flow. Default is `false`.
        pub allow_classic_flow: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the identity pool supports unauthenticated logins or not.
        pub allow_unauthenticated_identities: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN of the identity pool.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// An array of Amazon Cognito Identity user pools and their client IDs.
        pub cognito_identity_providers: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::cognito::IdentityPoolCognitoIdentityProvider>,
            >,
        >,
        /// The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your
        /// backend and the Cognito service to communicate about the developer provider.
        pub developer_provider_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Cognito Identity Pool name.
        pub identity_pool_name: pulumi_gestalt_rust::Output<String>,
        /// Set of OpendID Connect provider ARNs.
        pub openid_connect_provider_arns: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// An array of Amazon Resource Names (ARNs) of the SAML provider for your identity.
        pub saml_provider_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Key-Value pairs mapping provider names to provider app IDs.
        pub supported_login_providers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags to assign to the Identity Pool. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentityPoolArgs,
    ) -> IdentityPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_classic_flow_binding = args.allow_classic_flow.get_output(context);
        let allow_unauthenticated_identities_binding = args
            .allow_unauthenticated_identities
            .get_output(context);
        let cognito_identity_providers_binding = args
            .cognito_identity_providers
            .get_output(context);
        let developer_provider_name_binding = args
            .developer_provider_name
            .get_output(context);
        let identity_pool_name_binding = args.identity_pool_name.get_output(context);
        let openid_connect_provider_arns_binding = args
            .openid_connect_provider_arns
            .get_output(context);
        let saml_provider_arns_binding = args.saml_provider_arns.get_output(context);
        let supported_login_providers_binding = args
            .supported_login_providers
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/identityPool:IdentityPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowClassicFlow".into(),
                    value: allow_classic_flow_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowUnauthenticatedIdentities".into(),
                    value: allow_unauthenticated_identities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cognitoIdentityProviders".into(),
                    value: cognito_identity_providers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "developerProviderName".into(),
                    value: developer_provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityPoolName".into(),
                    value: identity_pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "openidConnectProviderArns".into(),
                    value: openid_connect_provider_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samlProviderArns".into(),
                    value: saml_provider_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "supportedLoginProviders".into(),
                    value: supported_login_providers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityPoolResult {
            allow_classic_flow: o.get_field("allowClassicFlow"),
            allow_unauthenticated_identities: o
                .get_field("allowUnauthenticatedIdentities"),
            arn: o.get_field("arn"),
            cognito_identity_providers: o.get_field("cognitoIdentityProviders"),
            developer_provider_name: o.get_field("developerProviderName"),
            identity_pool_name: o.get_field("identityPoolName"),
            openid_connect_provider_arns: o.get_field("openidConnectProviderArns"),
            saml_provider_arns: o.get_field("samlProviderArns"),
            supported_login_providers: o.get_field("supportedLoginProviders"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
