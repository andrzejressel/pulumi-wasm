/// Configures Https Redirection for a Lightsail Load Balancer. A valid Certificate must be attached to the load balancer in order to enable https redirection.
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
///   testLbHttpsRedirectionPolicy:
///     type: aws:lightsail:LbHttpsRedirectionPolicy
///     name: test
///     properties:
///       lbName: ${test.name}
///       enabled: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_https_redirection_policy` using the `lb_name` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbHttpsRedirectionPolicy:LbHttpsRedirectionPolicy test example-load-balancer
/// ```
pub mod lb_https_redirection_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbHttpsRedirectionPolicyArgs {
        /// The Https Redirection state of the load balancer. `true` to activate http to https redirection or `false` to deactivate http to https redirection.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The name of the load balancer to which you want to enable http to https redirection.
        #[builder(into)]
        pub lb_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LbHttpsRedirectionPolicyResult {
        /// The Https Redirection state of the load balancer. `true` to activate http to https redirection or `false` to deactivate http to https redirection.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the load balancer to which you want to enable http to https redirection.
        pub lb_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LbHttpsRedirectionPolicyArgs,
    ) -> LbHttpsRedirectionPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let lb_name_binding = args.lb_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/lbHttpsRedirectionPolicy:LbHttpsRedirectionPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "lbName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LbHttpsRedirectionPolicyResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            lb_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lbName").unwrap(),
            ),
        }
    }
}
