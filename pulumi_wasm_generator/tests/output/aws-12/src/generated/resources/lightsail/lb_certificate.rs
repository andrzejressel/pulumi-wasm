/// Creates a Lightsail load balancer Certificate resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Lb
///     properties:
///       name: test-load-balancer
///       healthCheckPath: /
///       instancePort: '80'
///       tags:
///         foo: bar
///   testLbCertificate:
///     type: aws:lightsail:LbCertificate
///     name: test
///     properties:
///       name: test-load-balancer-certificate
///       lbName: ${test.id}
///       domainName: test.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_certificate` using the id attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbCertificate:LbCertificate test example-load-balancer,example-load-balancer-certificate
/// ```
pub mod lb_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbCertificateArgs {
        /// The domain name (e.g., example.com) for your SSL/TLS certificate.
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The load balancer name where you want to create the SSL/TLS certificate.
        #[builder(into)]
        pub lb_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SSL/TLS certificate name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        #[builder(into, default)]
        pub subject_alternative_names: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LbCertificateResult {
        /// The ARN of the lightsail certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the instance was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The domain name (e.g., example.com) for your SSL/TLS certificate.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        pub domain_validation_records: pulumi_wasm_rust::Output<
            Vec<super::super::types::lightsail::LbCertificateDomainValidationRecord>,
        >,
        /// The load balancer name where you want to create the SSL/TLS certificate.
        pub lb_name: pulumi_wasm_rust::Output<String>,
        /// The SSL/TLS certificate name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        pub subject_alternative_names: pulumi_wasm_rust::Output<Vec<String>>,
        pub support_code: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LbCertificateArgs,
    ) -> LbCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let lb_name_binding = args.lb_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let subject_alternative_names_binding = args
            .subject_alternative_names
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/lbCertificate:LbCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subjectAlternativeNames".into(),
                    value: &subject_alternative_names_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "domainValidationRecords".into(),
                },
                register_interface::ResultField {
                    name: "lbName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subjectAlternativeNames".into(),
                },
                register_interface::ResultField {
                    name: "supportCode".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LbCertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            domain_validation_records: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainValidationRecords").unwrap(),
            ),
            lb_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lbName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subject_alternative_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subjectAlternativeNames").unwrap(),
            ),
            support_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportCode").unwrap(),
            ),
        }
    }
}
