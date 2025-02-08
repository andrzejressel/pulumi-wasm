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
#[allow(clippy::doc_lazy_continuation)]
pub mod trusted_token_issuer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustedTokenIssuerArgs {
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        #[builder(into, default)]
        pub client_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the trusted token issuer.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A block that specifies settings that apply to the trusted token issuer, these change depending on the type you specify in `trusted_token_issuer_type`. Documented below.
        #[builder(into, default)]
        pub trusted_token_issuer_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::ssoadmin::TrustedTokenIssuerTrustedTokenIssuerConfiguration,
            >,
        >,
        /// Specifies the type of the trusted token issuer. Valid values are `OIDC_JWT`
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub trusted_token_issuer_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TrustedTokenIssuerResult {
        /// ARN of the trusted token issuer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        pub client_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the trusted token issuer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A block that specifies settings that apply to the trusted token issuer, these change depending on the type you specify in `trusted_token_issuer_type`. Documented below.
        pub trusted_token_issuer_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::ssoadmin::TrustedTokenIssuerTrustedTokenIssuerConfiguration,
            >,
        >,
        /// Specifies the type of the trusted token issuer. Valid values are `OIDC_JWT`
        ///
        /// The following arguments are optional:
        pub trusted_token_issuer_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TrustedTokenIssuerArgs,
    ) -> TrustedTokenIssuerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let client_token_binding = args.client_token.get_output(context).get_inner();
        let instance_arn_binding = args.instance_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let trusted_token_issuer_configuration_binding = args
            .trusted_token_issuer_configuration
            .get_output(context)
            .get_inner();
        let trusted_token_issuer_type_binding = args
            .trusted_token_issuer_type
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/trustedTokenIssuer:TrustedTokenIssuer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TrustedTokenIssuerResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            client_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientToken"),
            ),
            instance_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            trusted_token_issuer_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedTokenIssuerConfiguration"),
            ),
            trusted_token_issuer_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedTokenIssuerType"),
            ),
        }
    }
}
