/// This resource represents a successful validation of an ACM certificate in concert
/// with other resources.
///
/// Most commonly, this resource is used together with `aws.route53.Record` and
/// `aws.acm.Certificate` to request a DNS validated certificate,
/// deploy the required validation records and wait for validation to complete.
///
/// > **WARNING:** This resource implements a part of the validation workflow. It does not represent a real-world entity in AWS, therefore changing or deleting this resource on its own has no immediate effect.
///
/// ## Example Usage
///
/// ### DNS Validation with Route 53
///
///
/// ### Alternative Domains DNS Validation with Route 53
///
///
/// ### Email Validation
///
/// In this situation, the resource is simply a waiter for manual email approval of ACM certificates.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = certificate::create(
///         "example",
///         CertificateArgs::builder()
///             .domain_name("example.com")
///             .validation_method("EMAIL")
///             .build_struct(),
///     );
///     let exampleCertificateValidation = certificate_validation::create(
///         "exampleCertificateValidation",
///         CertificateValidationArgs::builder()
///             .certificate_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod certificate_validation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateValidationArgs {
        /// ARN of the certificate that is being validated.
        #[builder(into)]
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// List of FQDNs that implement the validation. Only valid for DNS validation method ACM certificates. If this is set, the resource can implement additional sanity checks and has an explicit dependency on the resource that is implementing the validation
        #[builder(into, default)]
        pub validation_record_fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct CertificateValidationResult {
        /// ARN of the certificate that is being validated.
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// List of FQDNs that implement the validation. Only valid for DNS validation method ACM certificates. If this is set, the resource can implement additional sanity checks and has an explicit dependency on the resource that is implementing the validation
        pub validation_record_fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CertificateValidationArgs,
    ) -> CertificateValidationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_arn_binding = args.certificate_arn.get_inner();
        let validation_record_fqdns_binding = args.validation_record_fqdns.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:acm/certificateValidation:CertificateValidation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "validationRecordFqdns".into(),
                    value: &validation_record_fqdns_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "validationRecordFqdns".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateValidationResult {
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            validation_record_fqdns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationRecordFqdns").unwrap(),
            ),
        }
    }
}
