/// Configures Session Stickiness for a Lightsail Load Balancer.
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
///   testLbStickinessPolicy:
///     type: aws:lightsail:LbStickinessPolicy
///     name: test
///     properties:
///       lbName: ${test.name}
///       cookieDuration: 900
///       enabled: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_lb_stickiness_policy` using the `lb_name` attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/lbStickinessPolicy:LbStickinessPolicy test example-load-balancer
/// ```
pub mod lb_stickiness_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbStickinessPolicyArgs {
        /// The cookie duration in seconds. This determines the length of the session stickiness.
        #[builder(into)]
        pub cookie_duration: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The Session Stickiness state of the load balancer. `true` to activate session stickiness or `false` to deactivate session stickiness.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The name of the load balancer to which you want to enable session stickiness.
        #[builder(into)]
        pub lb_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LbStickinessPolicyResult {
        /// The cookie duration in seconds. This determines the length of the session stickiness.
        pub cookie_duration: pulumi_gestalt_rust::Output<i32>,
        /// The Session Stickiness state of the load balancer. `true` to activate session stickiness or `false` to deactivate session stickiness.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The name of the load balancer to which you want to enable session stickiness.
        pub lb_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LbStickinessPolicyArgs,
    ) -> LbStickinessPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cookie_duration_binding = args
            .cookie_duration
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let lb_name_binding = args.lb_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/lbStickinessPolicy:LbStickinessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cookieDuration".into(),
                    value: &cookie_duration_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "lbName".into(),
                    value: &lb_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LbStickinessPolicyResult {
            cookie_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cookieDuration"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            lb_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lbName"),
            ),
        }
    }
}
