/// Provides an SES domain identity resource
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
pub mod domain_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainIdentityArgs {
        /// The domain name to assign to SES
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainIdentityResult {
        /// The ARN of the domain identity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The domain name to assign to SES
        pub domain: pulumi_wasm_rust::Output<String>,
        /// A code which when added to the domain as a TXT record will signal to SES that the owner of the domain has authorized SES to act on their behalf. The domain identity will be in state "verification pending" until this is done. See the With Route53 Record example for how this might be achieved when the domain is hosted in Route 53 and managed by this provider.  Find out more about verifying domains in Amazon SES in the [AWS SES docs](http://docs.aws.amazon.com/ses/latest/DeveloperGuide/verify-domains.html).
        pub verification_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainIdentityArgs) -> DomainIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/domainIdentity:DomainIdentity".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "verificationToken".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainIdentityResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            verification_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verificationToken").unwrap(),
            ),
        }
    }
}