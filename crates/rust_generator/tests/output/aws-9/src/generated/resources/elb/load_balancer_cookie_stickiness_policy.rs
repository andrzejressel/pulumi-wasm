/// Provides a load balancer cookie stickiness policy, which allows an ELB to control the sticky session lifetime of the browser.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod load_balancer_cookie_stickiness_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerCookieStickinessPolicyArgs {
        /// The time period after which
        /// the session cookie should be considered stale, expressed in seconds.
        #[builder(into, default)]
        pub cookie_expiration_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The load balancer port to which the policy
        /// should be applied. This must be an active listener on the load
        /// balancer.
        #[builder(into)]
        pub lb_port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The load balancer to which the policy
        /// should be attached.
        #[builder(into)]
        pub load_balancer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the stickiness policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerCookieStickinessPolicyResult {
        /// The time period after which
        /// the session cookie should be considered stale, expressed in seconds.
        pub cookie_expiration_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The load balancer port to which the policy
        /// should be applied. This must be an active listener on the load
        /// balancer.
        pub lb_port: pulumi_gestalt_rust::Output<i32>,
        /// The load balancer to which the policy
        /// should be attached.
        pub load_balancer: pulumi_gestalt_rust::Output<String>,
        /// The name of the stickiness policy.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoadBalancerCookieStickinessPolicyArgs,
    ) -> LoadBalancerCookieStickinessPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cookie_expiration_period_binding = args
            .cookie_expiration_period
            .get_output(context);
        let lb_port_binding = args.lb_port.get_output(context);
        let load_balancer_binding = args.load_balancer.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elb/loadBalancerCookieStickinessPolicy:LoadBalancerCookieStickinessPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cookieExpirationPeriod".into(),
                    value: cookie_expiration_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lbPort".into(),
                    value: lb_port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancer".into(),
                    value: load_balancer_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoadBalancerCookieStickinessPolicyResult {
            cookie_expiration_period: o.get_field("cookieExpirationPeriod"),
            lb_port: o.get_field("lbPort"),
            load_balancer: o.get_field("loadBalancer"),
            name: o.get_field("name"),
        }
    }
}
