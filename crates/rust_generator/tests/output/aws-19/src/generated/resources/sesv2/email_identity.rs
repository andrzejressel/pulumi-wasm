/// Resource for managing an AWS SESv2 (Simple Email V2) Email Identity.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ### Email Address Identity
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityArgs {
        /// The configuration set to use by default when sending from this identity. Note that any configuration set defined in the email sending request takes precedence.
        #[builder(into, default)]
        pub configuration_set_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The configuration of the DKIM authentication settings for an email domain identity.
        #[builder(into, default)]
        pub dkim_signing_attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sesv2::EmailIdentityDkimSigningAttributes>,
        >,
        /// The email address or domain to verify.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub email_identity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityResult {
        /// ARN of the Email Identity.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration set to use by default when sending from this identity. Note that any configuration set defined in the email sending request takes precedence.
        pub configuration_set_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The configuration of the DKIM authentication settings for an email domain identity.
        pub dkim_signing_attributes: pulumi_gestalt_rust::Output<
            super::super::types::sesv2::EmailIdentityDkimSigningAttributes,
        >,
        /// The email address or domain to verify.
        ///
        /// The following arguments are optional:
        pub email_identity: pulumi_gestalt_rust::Output<String>,
        /// The email identity type. Valid values: `EMAIL_ADDRESS`, `DOMAIN`.
        pub identity_type: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies whether or not the identity is verified.
        pub verified_for_sending_status: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailIdentityArgs,
    ) -> EmailIdentityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_set_name_binding = args
            .configuration_set_name
            .get_output(context);
        let dkim_signing_attributes_binding = args
            .dkim_signing_attributes
            .get_output(context);
        let email_identity_binding = args.email_identity.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/emailIdentity:EmailIdentity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationSetName".into(),
                    value: &configuration_set_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dkimSigningAttributes".into(),
                    value: &dkim_signing_attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailIdentityResult {
            arn: o.get_field("arn"),
            configuration_set_name: o.get_field("configurationSetName"),
            dkim_signing_attributes: o.get_field("dkimSigningAttributes"),
            email_identity: o.get_field("emailIdentity"),
            identity_type: o.get_field("identityType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            verified_for_sending_status: o.get_field("verifiedForSendingStatus"),
        }
    }
}
