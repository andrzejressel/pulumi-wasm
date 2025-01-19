/// Provides a Load Balancer Listener Certificate resource.
///
/// This resource is for additional certificates and does not replace the default certificate on the listener.
///
/// > **Note:** `aws.alb.ListenerCertificate` is known as `aws.lb.ListenerCertificate`. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = certificate::create(
///         "example",
///         CertificateArgs::builder().build_struct(),
///     );
///     let exampleListenerCertificate = listener_certificate::create(
///         "exampleListenerCertificate",
///         ListenerCertificateArgs::builder()
///             .certificate_arn("${example.arn}")
///             .listener_arn("${frontEndListener.arn}")
///             .build_struct(),
///     );
///     let frontEnd = load_balancer::create(
///         "frontEnd",
///         LoadBalancerArgs::builder().build_struct(),
///     );
///     let frontEndListener = listener::create(
///         "frontEndListener",
///         ListenerArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Listener Certificates using the listener arn and certificate arn, separated by an underscore (`_`). For example:
///
/// ```sh
/// $ pulumi import aws:alb/listenerCertificate:ListenerCertificate example arn:aws:elasticloadbalancing:us-west-2:123456789012:listener/app/test/8e4497da625e2d8a/9ab28ade35828f96/67b3d2d36dd7c26b_arn:aws:iam::123456789012:server-certificate/tf-acc-test-6453083910015726063
/// ```
pub mod listener_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerCertificateArgs {
        /// The ARN of the certificate to attach to the listener.
        #[builder(into)]
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the listener to which to attach the certificate.
        #[builder(into)]
        pub listener_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ListenerCertificateResult {
        /// The ARN of the certificate to attach to the listener.
        pub certificate_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the listener to which to attach the certificate.
        pub listener_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ListenerCertificateArgs,
    ) -> ListenerCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_arn_binding = args.certificate_arn.get_inner();
        let listener_arn_binding = args.listener_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:alb/listenerCertificate:ListenerCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "listenerArn".into(),
                    value: &listener_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateArn".into(),
                },
                register_interface::ResultField {
                    name: "listenerArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ListenerCertificateResult {
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateArn").unwrap(),
            ),
            listener_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listenerArn").unwrap(),
            ),
        }
    }
}
