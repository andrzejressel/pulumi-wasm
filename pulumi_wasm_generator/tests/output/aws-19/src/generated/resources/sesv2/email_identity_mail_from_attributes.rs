/// Resource for managing an AWS SESv2 (Simple Email V2) Email Identity Mail From Attributes.
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
///     let exampleEmailIdentityMailFromAttributes = email_identity_mail_from_attributes::create(
///         "exampleEmailIdentityMailFromAttributes",
///         EmailIdentityMailFromAttributesArgs::builder()
///             .behavior_on_mx_failure("REJECT_MESSAGE")
///             .email_identity("${example.emailIdentity}")
///             .mail_from_domain("subdomain.${example.emailIdentity}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Email Identity Mail From Attributes using the `email_identity`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/emailIdentityMailFromAttributes:EmailIdentityMailFromAttributes example example.com
/// ```
pub mod email_identity_mail_from_attributes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailIdentityMailFromAttributesArgs {
        /// The action to take if the required MX record isn't found when you send an email. Valid values: `USE_DEFAULT_VALUE`, `REJECT_MESSAGE`.
        #[builder(into, default)]
        pub behavior_on_mx_failure: pulumi_wasm_rust::Output<Option<String>>,
        /// The verified email identity.
        #[builder(into)]
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// The custom MAIL FROM domain that you want the verified identity to use. Required if `behavior_on_mx_failure` is `REJECT_MESSAGE`.
        #[builder(into, default)]
        pub mail_from_domain: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EmailIdentityMailFromAttributesResult {
        /// The action to take if the required MX record isn't found when you send an email. Valid values: `USE_DEFAULT_VALUE`, `REJECT_MESSAGE`.
        pub behavior_on_mx_failure: pulumi_wasm_rust::Output<Option<String>>,
        /// The verified email identity.
        pub email_identity: pulumi_wasm_rust::Output<String>,
        /// The custom MAIL FROM domain that you want the verified identity to use. Required if `behavior_on_mx_failure` is `REJECT_MESSAGE`.
        pub mail_from_domain: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: EmailIdentityMailFromAttributesArgs,
    ) -> EmailIdentityMailFromAttributesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let behavior_on_mx_failure_binding = args.behavior_on_mx_failure.get_inner();
        let email_identity_binding = args.email_identity.get_inner();
        let mail_from_domain_binding = args.mail_from_domain.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sesv2/emailIdentityMailFromAttributes:EmailIdentityMailFromAttributes"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "behaviorOnMxFailure".into(),
                    value: &behavior_on_mx_failure_binding,
                },
                register_interface::ObjectField {
                    name: "emailIdentity".into(),
                    value: &email_identity_binding,
                },
                register_interface::ObjectField {
                    name: "mailFromDomain".into(),
                    value: &mail_from_domain_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "behaviorOnMxFailure".into(),
                },
                register_interface::ResultField {
                    name: "emailIdentity".into(),
                },
                register_interface::ResultField {
                    name: "mailFromDomain".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EmailIdentityMailFromAttributesResult {
            behavior_on_mx_failure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("behaviorOnMxFailure").unwrap(),
            ),
            email_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailIdentity").unwrap(),
            ),
            mail_from_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mailFromDomain").unwrap(),
            ),
        }
    }
}
