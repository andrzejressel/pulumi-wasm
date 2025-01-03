/// Resource for managing an AWS SESv2 (Simple Email V2) Email Identity.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ### Email Address Identity
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_identity::create(
///         "example",
///         EmailIdentityArgs::builder().email_identity("testing@example.com").build_struct(),
///     );
/// }
/// ```
///
/// ### Domain Identity
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_identity::create(
///         "example",
///         EmailIdentityArgs::builder().email_identity("example.com").build_struct(),
///     );
/// }
/// ```
///
/// ### Configuration Set
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration_set::create(
///         "example",
///         ConfigurationSetArgs::builder().configuration_set_name("example").build_struct(),
///     );
///     let exampleEmailIdentity = email_identity::create(
///         "exampleEmailIdentity",
///         EmailIdentityArgs::builder()
///             .configuration_set_name("${example.configurationSetName}")
///             .email_identity("example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### DKIM Signing Attributes (BYODKIM)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_identity::create(
///         "example",
///         EmailIdentityArgs::builder()
///             .dkim_signing_attributes(
///                 EmailIdentityDkimSigningAttributes::builder()
///                     .domainSigningPrivateKey(
///                         "MIIJKAIBAAKCAgEA2Se7p8zvnI4yh+Gh9j2rG5e2aRXjg03Y8saiupLnadPH9xvM...",
///                     )
///                     .domainSigningSelector("example")
///                     .build_struct(),
///             )
///             .email_identity("example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity using the `email_identity`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/emailIdentity:EmailIdentity example example.com
/// ```
pub mod email_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityArgs {
        /// The configuration set to use by default when sending from this identity. Note that any configuration set defined in the email sending request takes precedence.
        #[builder(into, default)]
        pub configuration_set_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration of the DKIM authentication settings for an email domain identity.
        #[builder(into, default)]
        pub dkim_signing_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::sesv2::EmailIdentityDkimSigningAttributes>,
        >,
        /// The email address or domain to verify.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityResult {
        /// ARN of the Email Identity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The configuration set to use by default when sending from this identity. Note that any configuration set defined in the email sending request takes precedence.
        pub configuration_set_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration of the DKIM authentication settings for an email domain identity.
        pub dkim_signing_attributes: pulumi_wasm_rust::Output<
            super::super::types::sesv2::EmailIdentityDkimSigningAttributes,
        >,
        /// The email address or domain to verify.
        ///
        /// The following arguments are optional:
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// The email identity type. Valid values: `EMAIL_ADDRESS`, `DOMAIN`.
        pub identity_type: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies whether or not the identity is verified.
        pub verified_for_sending_status: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EmailIdentityArgs) -> EmailIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_set_name_binding = args.configuration_set_name.get_inner();
        let dkim_signing_attributes_binding = args.dkim_signing_attributes.get_inner();
        let email_identity_binding = args.email_identity.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/emailIdentity:EmailIdentity".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationSetName".into(),
                    value: &configuration_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "dkimSigningAttributes".into(),
                    value: &dkim_signing_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configurationSetName".into(),
                },
                register_interface::ResultField {
                    name: "dkimSigningAttributes".into(),
                },
                register_interface::ResultField {
                    name: "emailIdentity".into(),
                },
                register_interface::ResultField {
                    name: "identityType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "verifiedForSendingStatus".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EmailIdentityResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationSetName").unwrap(),
            ),
            dkim_signing_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dkimSigningAttributes").unwrap(),
            ),
            email_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailIdentity").unwrap(),
            ),
            identity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            verified_for_sending_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedForSendingStatus").unwrap(),
            ),
        }
    }
}
