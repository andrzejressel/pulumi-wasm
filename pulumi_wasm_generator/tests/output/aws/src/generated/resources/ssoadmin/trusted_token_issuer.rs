/// Resource for managing an AWS SSO Admin Trusted Token Issuer.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleTrustedTokenIssuer:
///     type: aws:ssoadmin:TrustedTokenIssuer
///     name: example
///     properties:
///       name: example
///       instanceArn: ${example.arns[0]}
///       trustedTokenIssuerType: OIDC_JWT
///       trustedTokenIssuerConfiguration:
///         oidcJwtConfiguration:
///           claimAttributePath: email
///           identityStoreAttributePath: emails.value
///           issuerUrl: https://example.com
///           jwksRetrievalOption: OPEN_ID_DISCOVERY
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Trusted Token Issuer using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/trustedTokenIssuer:TrustedTokenIssuer example arn:aws:sso::123456789012:trustedTokenIssuer/ssoins-lu1ye3gew4mbc7ju/tti-2657c556-9707-11ee-b9d1-0242ac120002
/// ```
pub mod trusted_token_issuer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustedTokenIssuerArgs {
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        #[builder(into, default)]
        pub client_token: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the trusted token issuer.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A block that specifies settings that apply to the trusted token issuer, these change depending on the type you specify in `trusted_token_issuer_type`. Documented below.
        #[builder(into, default)]
        pub trusted_token_issuer_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ssoadmin::TrustedTokenIssuerTrustedTokenIssuerConfiguration,
            >,
        >,
        /// Specifies the type of the trusted token issuer. Valid values are `OIDC_JWT`
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub trusted_token_issuer_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TrustedTokenIssuerResult {
        /// ARN of the trusted token issuer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        pub client_token: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the trusted token issuer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A block that specifies settings that apply to the trusted token issuer, these change depending on the type you specify in `trusted_token_issuer_type`. Documented below.
        pub trusted_token_issuer_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::ssoadmin::TrustedTokenIssuerTrustedTokenIssuerConfiguration,
            >,
        >,
        /// Specifies the type of the trusted token issuer. Valid values are `OIDC_JWT`
        ///
        /// The following arguments are optional:
        pub trusted_token_issuer_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TrustedTokenIssuerArgs) -> TrustedTokenIssuerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_token_binding = args.client_token.get_inner();
        let instance_arn_binding = args.instance_arn.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let trusted_token_issuer_configuration_binding = args
            .trusted_token_issuer_configuration
            .get_inner();
        let trusted_token_issuer_type_binding = args
            .trusted_token_issuer_type
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/trustedTokenIssuer:TrustedTokenIssuer".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientToken".into(),
                    value: &client_token_binding,
                },
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trustedTokenIssuerConfiguration".into(),
                    value: &trusted_token_issuer_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "trustedTokenIssuerType".into(),
                    value: &trusted_token_issuer_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clientToken".into(),
                },
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "trustedTokenIssuerConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "trustedTokenIssuerType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrustedTokenIssuerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            client_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientToken").unwrap(),
            ),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            trusted_token_issuer_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedTokenIssuerConfiguration").unwrap(),
            ),
            trusted_token_issuer_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedTokenIssuerType").unwrap(),
            ),
        }
    }
}
