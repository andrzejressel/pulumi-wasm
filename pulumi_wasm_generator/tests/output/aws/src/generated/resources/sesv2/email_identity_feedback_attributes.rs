/// Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Feedback Attributes.
///
/// ## Example Usage
///
/// ### Basic Usage
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
pub mod email_identity_feedback_attributes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityFeedbackAttributesArgs {
        /// Sets the feedback forwarding configuration for the identity.
        #[builder(into, default)]
        pub email_forwarding_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The email identity.
        #[builder(into)]
        pub email_identity: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityFeedbackAttributesResult {
        /// Sets the feedback forwarding configuration for the identity.
        pub email_forwarding_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The email identity.
        pub email_identity: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: EmailIdentityFeedbackAttributesArgs,
    ) -> EmailIdentityFeedbackAttributesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_forwarding_enabled_binding = args.email_forwarding_enabled.get_inner();
        let email_identity_binding = args.email_identity.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/emailIdentityFeedbackAttributes:EmailIdentityFeedbackAttributes"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "emailForwardingEnabled".into(),
                    value: &email_forwarding_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "emailForwardingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "emailIdentity".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EmailIdentityFeedbackAttributesResult {
            email_forwarding_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailForwardingEnabled").unwrap(),
            ),
            email_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailIdentity").unwrap(),
            ),
        }
    }
}
