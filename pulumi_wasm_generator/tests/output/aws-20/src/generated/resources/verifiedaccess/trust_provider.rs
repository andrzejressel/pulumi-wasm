/// Resource for managing a Verified Access Trust Provider.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trust_provider::create(
///         "example",
///         TrustProviderArgs::builder()
///             .policy_reference_name("example")
///             .trust_provider_type("user")
///             .user_trust_provider_type("iam-identity-center")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer Workflows using the  `id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedaccess/trustProvider:TrustProvider example vatp-8012925589
/// ```
pub mod trust_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustProviderArgs {
        /// A description for the AWS Verified Access trust provider.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A block of options for device identity based trust providers.
        #[builder(into, default)]
        pub device_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::TrustProviderDeviceOptions>,
        >,
        /// The type of device-based trust provider.
        #[builder(into, default)]
        pub device_trust_provider_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The OpenID Connect details for an oidc-type, user-identity based trust provider.
        #[builder(into, default)]
        pub oidc_options: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::TrustProviderOidcOptions>,
        >,
        /// The identifier to be used when working with policy rules.
        #[builder(into)]
        pub policy_reference_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of trust provider can be either user or device-based.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub trust_provider_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of user-based trust provider.
        #[builder(into, default)]
        pub user_trust_provider_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TrustProviderResult {
        /// A description for the AWS Verified Access trust provider.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A block of options for device identity based trust providers.
        pub device_options: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::TrustProviderDeviceOptions>,
        >,
        /// The type of device-based trust provider.
        pub device_trust_provider_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The OpenID Connect details for an oidc-type, user-identity based trust provider.
        pub oidc_options: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::TrustProviderOidcOptions>,
        >,
        /// The identifier to be used when working with policy rules.
        pub policy_reference_name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of trust provider can be either user or device-based.
        ///
        /// The following arguments are optional:
        pub trust_provider_type: pulumi_wasm_rust::Output<String>,
        /// The type of user-based trust provider.
        pub user_trust_provider_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TrustProviderArgs,
    ) -> TrustProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let device_options_binding = args.device_options.get_output(context).get_inner();
        let device_trust_provider_type_binding = args
            .device_trust_provider_type
            .get_output(context)
            .get_inner();
        let oidc_options_binding = args.oidc_options.get_output(context).get_inner();
        let policy_reference_name_binding = args
            .policy_reference_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let trust_provider_type_binding = args
            .trust_provider_type
            .get_output(context)
            .get_inner();
        let user_trust_provider_type_binding = args
            .user_trust_provider_type
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedaccess/trustProvider:TrustProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "deviceOptions".into(),
                    value: &device_options_binding,
                },
                register_interface::ObjectField {
                    name: "deviceTrustProviderType".into(),
                    value: &device_trust_provider_type_binding,
                },
                register_interface::ObjectField {
                    name: "oidcOptions".into(),
                    value: &oidc_options_binding,
                },
                register_interface::ObjectField {
                    name: "policyReferenceName".into(),
                    value: &policy_reference_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trustProviderType".into(),
                    value: &trust_provider_type_binding,
                },
                register_interface::ObjectField {
                    name: "userTrustProviderType".into(),
                    value: &user_trust_provider_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "deviceOptions".into(),
                },
                register_interface::ResultField {
                    name: "deviceTrustProviderType".into(),
                },
                register_interface::ResultField {
                    name: "oidcOptions".into(),
                },
                register_interface::ResultField {
                    name: "policyReferenceName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "trustProviderType".into(),
                },
                register_interface::ResultField {
                    name: "userTrustProviderType".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrustProviderResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            device_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceOptions").unwrap(),
            ),
            device_trust_provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceTrustProviderType").unwrap(),
            ),
            oidc_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oidcOptions").unwrap(),
            ),
            policy_reference_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyReferenceName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            trust_provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustProviderType").unwrap(),
            ),
            user_trust_provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userTrustProviderType").unwrap(),
            ),
        }
    }
}
