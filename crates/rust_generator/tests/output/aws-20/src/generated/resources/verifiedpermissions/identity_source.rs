/// Resource for managing an AWS Verified Permissions Identity Source.
///
/// ## Example Usage
///
/// ### Cognito User Pool Configuration Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy_store::create(
///         "example",
///         PolicyStoreArgs::builder()
///             .validation_settings(
///                 PolicyStoreValidationSettings::builder().mode("STRICT").build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleIdentitySource = identity_source::create(
///         "exampleIdentitySource",
///         IdentitySourceArgs::builder()
///             .configuration(
///                 IdentitySourceConfiguration::builder()
///                     .cognitoUserPoolConfiguration(
///                         IdentitySourceConfigurationCognitoUserPoolConfiguration::builder()
///                             .clientIds(vec!["${exampleUserPoolClient.id}",])
///                             .userPoolArn("${exampleUserPool.arn}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .policy_store_id("${example.id}")
///             .build_struct(),
///     );
///     let exampleUserPool = user_pool::create(
///         "exampleUserPool",
///         UserPoolArgs::builder().name("example").build_struct(),
///     );
///     let exampleUserPoolClient = user_pool_client::create(
///         "exampleUserPoolClient",
///         UserPoolClientArgs::builder()
///             .explicit_auth_flows(vec!["ADMIN_NO_SRP_AUTH",])
///             .name("example")
///             .user_pool_id("${exampleUserPool.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### OpenID Connect Configuration Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy_store::create(
///         "example",
///         PolicyStoreArgs::builder()
///             .validation_settings(
///                 PolicyStoreValidationSettings::builder().mode("STRICT").build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleIdentitySource = identity_source::create(
///         "exampleIdentitySource",
///         IdentitySourceArgs::builder()
///             .configuration(
///                 IdentitySourceConfiguration::builder()
///                     .openIdConnectConfiguration(
///                         IdentitySourceConfigurationOpenIdConnectConfiguration::builder()
///                             .entityIdPrefix("MyOIDCProvider")
///                             .groupConfiguration(
///                                 IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfiguration::builder()
///                                     .groupClaim("groups")
///                                     .groupEntityType("MyCorp::UserGroup")
///                                     .build_struct(),
///                             )
///                             .issuer("https://auth.example.com")
///                             .tokenSelection(
///                                 IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection::builder()
///                                     .accessTokenOnly(
///                                         IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly::builder()
///                                             .audiences(vec!["https://myapp.example.com",])
///                                             .principalIdClaim("sub")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .policy_store_id("${example.id}")
///             .principal_entity_type("MyCorp::User")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Identity Source using the `policy_store_id:identity_source_id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedpermissions/identitySource:IdentitySource example policy-store-id-12345678:identity-source-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentitySourceArgs {
        /// Specifies the details required to communicate with the identity provider (IdP) associated with this identity source. See Configuration below.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedpermissions::IdentitySourceConfiguration>,
        >,
        /// Specifies the ID of the policy store in which you want to store this identity source.
        #[builder(into)]
        pub policy_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the namespace and data type of the principals generated for identities authenticated by the new identity source.
        #[builder(into, default)]
        pub principal_entity_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IdentitySourceResult {
        /// Specifies the details required to communicate with the identity provider (IdP) associated with this identity source. See Configuration below.
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::verifiedpermissions::IdentitySourceConfiguration>,
        >,
        /// Specifies the ID of the policy store in which you want to store this identity source.
        pub policy_store_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the namespace and data type of the principals generated for identities authenticated by the new identity source.
        pub principal_entity_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IdentitySourceArgs,
    ) -> IdentitySourceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_binding = args.configuration.get_output(context);
        let policy_store_id_binding = args.policy_store_id.get_output(context);
        let principal_entity_type_binding = args
            .principal_entity_type
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/identitySource:IdentitySource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyStoreId".into(),
                    value: policy_store_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalEntityType".into(),
                    value: principal_entity_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentitySourceResult {
            configuration: o.get_field("configuration"),
            policy_store_id: o.get_field("policyStoreId"),
            principal_entity_type: o.get_field("principalEntityType"),
        }
    }
}
