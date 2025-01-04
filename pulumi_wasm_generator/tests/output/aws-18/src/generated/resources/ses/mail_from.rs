/// Provides an SES domain MAIL FROM resource.
///
/// > **NOTE:** For the MAIL FROM domain to be fully usable, this resource should be paired with the aws.ses.DomainIdentity resource. To validate the MAIL FROM domain, a DNS MX record is required. To pass SPF checks, a DNS TXT record may also be required. See the [Amazon SES MAIL FROM documentation](https://docs.aws.amazon.com/ses/latest/dg/mail-from.html) for more information.
///
/// ## Example Usage
///
/// ### Domain Identity MAIL FROM
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ses:MailFrom
///     properties:
///       domain: ${exampleDomainIdentity.domain}
///       mailFromDomain: bounce.${exampleDomainIdentity.domain}
///   # Example SES Domain Identity
///   exampleDomainIdentity:
///     type: aws:ses:DomainIdentity
///     name: example
///     properties:
///       domain: example.com
///   # Example Route53 MX record
///   exampleSesDomainMailFromMx:
///     type: aws:route53:Record
///     name: example_ses_domain_mail_from_mx
///     properties:
///       zoneId: ${exampleAwsRoute53Zone.id}
///       name: ${example.mailFromDomain}
///       type: MX
///       ttl: '600'
///       records: # Change to the region in which `aws_ses_domain_identity.example` is created
///         - 10 feedback-smtp.us-east-1.amazonses.com
///   # Example Route53 TXT record for SPF
///   exampleSesDomainMailFromTxt:
///     type: aws:route53:Record
///     name: example_ses_domain_mail_from_txt
///     properties:
///       zoneId: ${exampleAwsRoute53Zone.id}
///       name: ${example.mailFromDomain}
///       type: TXT
///       ttl: '600'
///       records:
///         - v=spf1 include:amazonses.com ~all
/// ```
///
/// ### Email Identity MAIL FROM
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_identity::create(
///         "example",
///         EmailIdentityArgs::builder().email("user@example.com").build_struct(),
///     );
///     let exampleMailFrom = mail_from::create(
///         "exampleMailFrom",
///         MailFromArgs::builder()
///             .domain("${example.email}")
///             .mail_from_domain("mail.example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MAIL FROM domain using the `domain` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:ses/mailFrom:MailFrom example example.com
/// ```
pub mod mail_from {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MailFromArgs {
        /// The action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. Defaults to `UseDefaultValue`. See the [SES API documentation](https://docs.aws.amazon.com/ses/latest/APIReference/API_SetIdentityMailFromDomain.html) for more information.
        #[builder(into, default)]
        pub behavior_on_mx_failure: pulumi_wasm_rust::Output<Option<String>>,
        /// Verified domain name or email identity to generate DKIM tokens for.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Subdomain (of above domain) which is to be used as MAIL FROM address (Required for DMARC validation)
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub mail_from_domain: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MailFromResult {
        /// The action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. Defaults to `UseDefaultValue`. See the [SES API documentation](https://docs.aws.amazon.com/ses/latest/APIReference/API_SetIdentityMailFromDomain.html) for more information.
        pub behavior_on_mx_failure: pulumi_wasm_rust::Output<Option<String>>,
        /// Verified domain name or email identity to generate DKIM tokens for.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Subdomain (of above domain) which is to be used as MAIL FROM address (Required for DMARC validation)
        ///
        /// The following arguments are optional:
        pub mail_from_domain: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MailFromArgs) -> MailFromResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let behavior_on_mx_failure_binding = args.behavior_on_mx_failure.get_inner();
        let domain_binding = args.domain.get_inner();
        let mail_from_domain_binding = args.mail_from_domain.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/mailFrom:MailFrom".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "behaviorOnMxFailure".into(),
                    value: &behavior_on_mx_failure_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
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
                    name: "domain".into(),
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
        MailFromResult {
            behavior_on_mx_failure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("behaviorOnMxFailure").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            mail_from_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mailFromDomain").unwrap(),
            ),
        }
    }
}
