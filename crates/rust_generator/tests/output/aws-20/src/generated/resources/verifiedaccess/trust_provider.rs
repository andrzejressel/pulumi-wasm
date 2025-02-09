/// Resource for managing a Verified Access Trust Provider.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trust_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustProviderArgs {
        /// A description for the AWS Verified Access trust provider.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A block of options for device identity based trust providers.
        #[builder(into, default)]
        pub device_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::TrustProviderDeviceOptions>,
        >,
        /// The type of device-based trust provider.
        #[builder(into, default)]
        pub device_trust_provider_type: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The OpenID Connect details for an oidc-type, user-identity based trust provider.
        #[builder(into, default)]
        pub oidc_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::TrustProviderOidcOptions>,
        >,
        /// The identifier to be used when working with policy rules.
        #[builder(into)]
        pub policy_reference_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of trust provider can be either user or device-based.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub trust_provider_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of user-based trust provider.
        #[builder(into, default)]
        pub user_trust_provider_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TrustProviderResult {
        /// A description for the AWS Verified Access trust provider.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A block of options for device identity based trust providers.
        pub device_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::verifiedaccess::TrustProviderDeviceOptions>,
        >,
        /// The type of device-based trust provider.
        pub device_trust_provider_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The OpenID Connect details for an oidc-type, user-identity based trust provider.
        pub oidc_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::verifiedaccess::TrustProviderOidcOptions>,
        >,
        /// The identifier to be used when working with policy rules.
        pub policy_reference_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of trust provider can be either user or device-based.
        ///
        /// The following arguments are optional:
        pub trust_provider_type: pulumi_gestalt_rust::Output<String>,
        /// The type of user-based trust provider.
        pub user_trust_provider_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustProviderArgs,
    ) -> TrustProviderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let device_options_binding = args.device_options.get_output(context);
        let device_trust_provider_type_binding = args
            .device_trust_provider_type
            .get_output(context);
        let oidc_options_binding = args.oidc_options.get_output(context);
        let policy_reference_name_binding = args
            .policy_reference_name
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let trust_provider_type_binding = args.trust_provider_type.get_output(context);
        let user_trust_provider_type_binding = args
            .user_trust_provider_type
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedaccess/trustProvider:TrustProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceOptions".into(),
                    value: device_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceTrustProviderType".into(),
                    value: device_trust_provider_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "oidcOptions".into(),
                    value: oidc_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyReferenceName".into(),
                    value: policy_reference_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustProviderType".into(),
                    value: trust_provider_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userTrustProviderType".into(),
                    value: user_trust_provider_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrustProviderResult {
            description: o.get_field("description"),
            device_options: o.get_field("deviceOptions"),
            device_trust_provider_type: o.get_field("deviceTrustProviderType"),
            oidc_options: o.get_field("oidcOptions"),
            policy_reference_name: o.get_field("policyReferenceName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            trust_provider_type: o.get_field("trustProviderType"),
            user_trust_provider_type: o.get_field("userTrustProviderType"),
        }
    }
}
