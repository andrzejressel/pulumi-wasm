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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LbStickinessPolicyArgs,
    ) -> LbStickinessPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cookie_duration_binding = args.cookie_duration.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let lb_name_binding = args.lb_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/lbStickinessPolicy:LbStickinessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cookieDuration".into(),
                    value: cookie_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lbName".into(),
                    value: lb_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LbStickinessPolicyResult {
            cookie_duration: o.get_field("cookieDuration"),
            enabled: o.get_field("enabled"),
            lb_name: o.get_field("lbName"),
        }
    }
}
