/// Resource for managing an AWS Verified Permissions Identity Source.
///
/// ## Example Usage
///
/// ### Cognito User Pool Configuration Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod identity_source {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentitySourceArgs {
        /// Specifies the details required to communicate with the identity provider (IdP) associated with this identity source. See Configuration below.
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedpermissions::IdentitySourceConfiguration>,
        >,
        /// Specifies the ID of the policy store in which you want to store this identity source.
        #[builder(into)]
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the namespace and data type of the principals generated for identities authenticated by the new identity source.
        #[builder(into, default)]
        pub principal_entity_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IdentitySourceResult {
        /// Specifies the details required to communicate with the identity provider (IdP) associated with this identity source. See Configuration below.
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedpermissions::IdentitySourceConfiguration>,
        >,
        /// Specifies the ID of the policy store in which you want to store this identity source.
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the namespace and data type of the principals generated for identities authenticated by the new identity source.
        pub principal_entity_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: IdentitySourceArgs) -> IdentitySourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let policy_store_id_binding = args.policy_store_id.get_inner();
        let principal_entity_type_binding = args.principal_entity_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/identitySource:IdentitySource".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "policyStoreId".into(),
                    value: &policy_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "principalEntityType".into(),
                    value: &principal_entity_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "policyStoreId".into(),
                },
                register_interface::ResultField {
                    name: "principalEntityType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentitySourceResult {
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            policy_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyStoreId").unwrap(),
            ),
            principal_entity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalEntityType").unwrap(),
            ),
        }
    }
}