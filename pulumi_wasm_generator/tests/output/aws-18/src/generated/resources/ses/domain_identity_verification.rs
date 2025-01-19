/// Represents a successful verification of an SES domain identity.
///
/// Most commonly, this resource is used together with `aws.route53.Record` and
/// `aws.ses.DomainIdentity` to request an SES domain identity,
/// deploy the required DNS verification records, and wait for verification to complete.
///
/// > **WARNING:** This resource implements a part of the verification workflow. It does not represent a real-world entity in AWS, therefore changing or deleting this resource on its own has no immediate effect.
///
/// ## Example Usage
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
///       zoneId: ${exampleAwsRoute53Zone.zoneId}
///       name: _amazonses.${example.id}
///       type: TXT
///       ttl: '600'
///       records:
///         - ${example.verificationToken}
///   exampleVerification:
///     type: aws:ses:DomainIdentityVerification
///     name: example_verification
///     properties:
///       domain: ${example.id}
///     options:
///       dependsOn:
///         - ${exampleAmazonsesVerificationRecord}
/// ```
pub mod domain_identity_verification {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainIdentityVerificationArgs {
        /// The domain name of the SES domain identity to verify.
        #[builder(into)]
        pub domain: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainIdentityVerificationResult {
        /// The ARN of the domain identity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The domain name of the SES domain identity to verify.
        pub domain: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DomainIdentityVerificationArgs,
    ) -> DomainIdentityVerificationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/domainIdentityVerification:DomainIdentityVerification"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainIdentityVerificationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
        }
    }
}
