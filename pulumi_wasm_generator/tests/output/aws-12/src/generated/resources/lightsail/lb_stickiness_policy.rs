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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LbStickinessPolicyArgs {
        /// The cookie duration in seconds. This determines the length of the session stickiness.
        #[builder(into)]
        pub cookie_duration: pulumi_wasm_rust::Output<i32>,
        /// The Session Stickiness state of the load balancer. `true` to activate session stickiness or `false` to deactivate session stickiness.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the load balancer to which you want to enable session stickiness.
        #[builder(into)]
        pub lb_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LbStickinessPolicyResult {
        /// The cookie duration in seconds. This determines the length of the session stickiness.
        pub cookie_duration: pulumi_wasm_rust::Output<i32>,
        /// The Session Stickiness state of the load balancer. `true` to activate session stickiness or `false` to deactivate session stickiness.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the load balancer to which you want to enable session stickiness.
        pub lb_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LbStickinessPolicyArgs) -> LbStickinessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cookie_duration_binding = args.cookie_duration.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let lb_name_binding = args.lb_name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "cookieDuration".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "lbName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LbStickinessPolicyResult {
            cookie_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cookieDuration").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            lb_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lbName").unwrap(),
            ),
        }
    }
}
