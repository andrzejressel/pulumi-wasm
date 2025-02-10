/// Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Feedback Attributes.
///
/// ## Example Usage
///
/// ### Basic Usage
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
///     let exampleEmailIdentityFeedbackAttributes = email_identity_feedback_attributes::create(
///         "exampleEmailIdentityFeedbackAttributes",
///         EmailIdentityFeedbackAttributesArgs::builder()
///             .email_forwarding_enabled(true)
///             .email_identity("${example.emailIdentity}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity Feedback Attributes using the `email_identity`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/emailIdentityFeedbackAttributes:EmailIdentityFeedbackAttributes example example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_identity_feedback_attributes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityFeedbackAttributesArgs {
        /// Sets the feedback forwarding configuration for the identity.
        #[builder(into, default)]
        pub email_forwarding_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The email identity.
        #[builder(into)]
        pub email_identity: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityFeedbackAttributesResult {
        /// Sets the feedback forwarding configuration for the identity.
        pub email_forwarding_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The email identity.
        pub email_identity: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailIdentityFeedbackAttributesArgs,
    ) -> EmailIdentityFeedbackAttributesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let email_forwarding_enabled_binding = args
            .email_forwarding_enabled
            .get_output(context);
        let email_identity_binding = args.email_identity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/emailIdentityFeedbackAttributes:EmailIdentityFeedbackAttributes"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailForwardingEnabled".into(),
                    value: email_forwarding_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailIdentity".into(),
                    value: email_identity_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailIdentityFeedbackAttributesResult {
            email_forwarding_enabled: o.get_field("emailForwardingEnabled"),
            email_identity: o.get_field("emailIdentity"),
        }
    }
}
