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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mail_from {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MailFromArgs {
        /// The action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. Defaults to `UseDefaultValue`. See the [SES API documentation](https://docs.aws.amazon.com/ses/latest/APIReference/API_SetIdentityMailFromDomain.html) for more information.
        #[builder(into, default)]
        pub behavior_on_mx_failure: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Verified domain name or email identity to generate DKIM tokens for.
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Subdomain (of above domain) which is to be used as MAIL FROM address (Required for DMARC validation)
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub mail_from_domain: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MailFromResult {
        /// The action that you want Amazon SES to take if it cannot successfully read the required MX record when you send an email. Defaults to `UseDefaultValue`. See the [SES API documentation](https://docs.aws.amazon.com/ses/latest/APIReference/API_SetIdentityMailFromDomain.html) for more information.
        pub behavior_on_mx_failure: pulumi_gestalt_rust::Output<Option<String>>,
        /// Verified domain name or email identity to generate DKIM tokens for.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// Subdomain (of above domain) which is to be used as MAIL FROM address (Required for DMARC validation)
        ///
        /// The following arguments are optional:
        pub mail_from_domain: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MailFromArgs,
    ) -> MailFromResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let behavior_on_mx_failure_binding = args
            .behavior_on_mx_failure
            .get_output(context);
        let domain_binding = args.domain.get_output(context);
        let mail_from_domain_binding = args.mail_from_domain.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/mailFrom:MailFrom".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "behaviorOnMxFailure".into(),
                    value: behavior_on_mx_failure_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mailFromDomain".into(),
                    value: mail_from_domain_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MailFromResult {
            behavior_on_mx_failure: o.get_field("behaviorOnMxFailure"),
            domain: o.get_field("domain"),
            mail_from_domain: o.get_field("mailFromDomain"),
        }
    }
}
