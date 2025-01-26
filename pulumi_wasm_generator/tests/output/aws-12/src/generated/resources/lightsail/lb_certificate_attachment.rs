/// Attaches a Lightsail Load Balancer Certificate to a Lightsail Load Balancer.
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
///   testLbCertificateAttachment:
///     type: aws:lightsail:LbCertificateAttachment
///     name: test
///     properties:
///       lbName: ${test.name}
///       certificateName: ${testLbCertificate.name}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_certificate_attachment` using the name attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbCertificateAttachment:LbCertificateAttachment test example-load-balancer,example-certificate
/// ```
pub mod lb_certificate_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbCertificateAttachmentArgs {
        /// The name of your SSL/TLS certificate.
        #[builder(into)]
        pub certificate_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the load balancer to which you want to associate the SSL/TLS certificate.
        #[builder(into)]
        pub lb_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LbCertificateAttachmentResult {
        /// The name of your SSL/TLS certificate.
        pub certificate_name: pulumi_wasm_rust::Output<String>,
        /// The name of the load balancer to which you want to associate the SSL/TLS certificate.
        pub lb_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LbCertificateAttachmentArgs,
    ) -> LbCertificateAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_name_binding = args
            .certificate_name
            .get_output(context)
            .get_inner();
        let lb_name_binding = args.lb_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/lbCertificateAttachment:LbCertificateAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding,
                },
                register_interface::ObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LbCertificateAttachmentResult {
            certificate_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateName"),
            ),
            lb_name: pulumi_wasm_rust::__private::into_domain(o.extract_field("lbName")),
        }
    }
}
