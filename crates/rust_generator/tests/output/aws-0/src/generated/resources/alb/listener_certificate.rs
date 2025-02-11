/// Provides a Load Balancer Listener Certificate resource.
///
/// This resource is for additional certificates and does not replace the default certificate on the listener.
///
/// > **Note:** `aws.alb.ListenerCertificate` is known as `aws.lb.ListenerCertificate`. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod listener_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListenerCertificateArgs {
        /// The ARN of the certificate to attach to the listener.
        #[builder(into)]
        pub certificate_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ARN of the listener to which to attach the certificate.
        #[builder(into)]
        pub listener_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ListenerCertificateResult {
        /// The ARN of the certificate to attach to the listener.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the listener to which to attach the certificate.
        pub listener_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListenerCertificateArgs,
    ) -> ListenerCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_arn_binding = args.certificate_arn.get_output(context);
        let listener_arn_binding = args.listener_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:alb/listenerCertificate:ListenerCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listenerArn".into(),
                    value: &listener_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ListenerCertificateResult {
            certificate_arn: o.get_field("certificateArn"),
            listener_arn: o.get_field("listenerArn"),
        }
    }
}
