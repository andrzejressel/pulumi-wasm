/// Provides a load balancer cookie stickiness policy, which allows an ELB to control the sticky session lifetime of the browser.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = load_balancer_cookie_stickiness_policy::create(
///         "foo",
///         LoadBalancerCookieStickinessPolicyArgs::builder()
///             .cookie_expiration_period(600)
///             .lb_port(80)
///             .load_balancer("${lb.id}")
///             .name("foo-policy")
///             .build_struct(),
///     );
///     let lb = load_balancer::create(
///         "lb",
///         LoadBalancerArgs::builder()
///             .availability_zones(vec!["us-east-1a",])
///             .listeners(
///                 vec![
///                     LoadBalancerListener::builder().instancePort(8000)
///                     .instanceProtocol("http").lbPort(80).lbProtocol("http")
///                     .build_struct(),
///                 ],
///             )
///             .name("test-lb")
///             .build_struct(),
///     );
/// }
/// ```
pub mod load_balancer_cookie_stickiness_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerCookieStickinessPolicyArgs {
        /// The time period after which
        /// the session cookie should be considered stale, expressed in seconds.
        #[builder(into, default)]
        pub cookie_expiration_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The load balancer port to which the policy
        /// should be applied. This must be an active listener on the load
        /// balancer.
        #[builder(into)]
        pub lb_port: pulumi_wasm_rust::Output<i32>,
        /// The load balancer to which the policy
        /// should be attached.
        #[builder(into)]
        pub load_balancer: pulumi_wasm_rust::Output<String>,
        /// The name of the stickiness policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerCookieStickinessPolicyResult {
        /// The time period after which
        /// the session cookie should be considered stale, expressed in seconds.
        pub cookie_expiration_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The load balancer port to which the policy
        /// should be applied. This must be an active listener on the load
        /// balancer.
        pub lb_port: pulumi_wasm_rust::Output<i32>,
        /// The load balancer to which the policy
        /// should be attached.
        pub load_balancer: pulumi_wasm_rust::Output<String>,
        /// The name of the stickiness policy.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LoadBalancerCookieStickinessPolicyArgs,
    ) -> LoadBalancerCookieStickinessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cookie_expiration_period_binding = args.cookie_expiration_period.get_inner();
        let lb_port_binding = args.lb_port.get_inner();
        let load_balancer_binding = args.load_balancer.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elb/loadBalancerCookieStickinessPolicy:LoadBalancerCookieStickinessPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cookieExpirationPeriod".into(),
                    value: &cookie_expiration_period_binding,
                },
                register_interface::ObjectField {
                    name: "lbPort".into(),
                    value: &lb_port_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancer".into(),
                    value: &load_balancer_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cookieExpirationPeriod".into(),
                },
                register_interface::ResultField {
                    name: "lbPort".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancer".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoadBalancerCookieStickinessPolicyResult {
            cookie_expiration_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cookieExpirationPeriod").unwrap(),
            ),
            lb_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lbPort").unwrap(),
            ),
            load_balancer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancer").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}