/// Provides an SES domain identity resource
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
///     let example = domain_identity::create(
///         "example",
///         DomainIdentityArgs::builder().domain("example.com").build_struct(),
///     );
/// }
/// ```
///
/// ### With Route53 Record
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ses:DomainIdentity
///     properties:
///       domain: example.com
///   exampleAmazonsesVerificationRecord:
///     type: aws:route53:Record
///     name: example_amazonses_verification_record
///     properties:
///       zoneId: ABCDEFGHIJ123
///       name: _amazonses.example.com
///       type: TXT
///       ttl: '600'
///       records:
///         - ${example.verificationToken}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES domain identities using the domain name. For example:
///
/// ```sh
/// $ pulumi import aws:ses/domainIdentity:DomainIdentity example example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainIdentityArgs {
        /// The domain name to assign to SES
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainIdentityResult {
        /// The ARN of the domain identity.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The domain name to assign to SES
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// A code which when added to the domain as a TXT record will signal to SES that the owner of the domain has authorized SES to act on their behalf. The domain identity will be in state "verification pending" until this is done. See the With Route53 Record example for how this might be achieved when the domain is hosted in Route 53 and managed by this provider.  Find out more about verifying domains in Amazon SES in the [AWS SES docs](http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html).
        pub verification_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainIdentityArgs,
    ) -> DomainIdentityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/domainIdentity:DomainIdentity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainIdentityResult {
            arn: o.get_field("arn"),
            domain: o.get_field("domain"),
            verification_token: o.get_field("verificationToken"),
        }
    }
}
