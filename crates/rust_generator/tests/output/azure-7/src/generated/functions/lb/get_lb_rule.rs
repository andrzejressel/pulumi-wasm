#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_lb_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLbRuleArgs {
        /// The ID of the Load Balancer Rule.
        #[builder(into)]
        pub loadbalancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Load Balancer Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLbRuleResult {
        /// A reference to a Backend Address Pool over which this Load Balancing Rule operates.
        pub backend_address_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The port used for internal connections on the endpoint.
        pub backend_port: pulumi_gestalt_rust::Output<i32>,
        /// If outbound SNAT is enabled for this Load Balancer Rule.
        pub disable_outbound_snat: pulumi_gestalt_rust::Output<bool>,
        /// If Floating IPs are enabled for this Load Balancer Rule
        pub enable_floating_ip: pulumi_gestalt_rust::Output<bool>,
        /// If TCP Reset is enabled for this Load Balancer Rule.
        pub enable_tcp_reset: pulumi_gestalt_rust::Output<bool>,
        /// The name of the frontend IP configuration to which the rule is associated.
        pub frontend_ip_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The port for the external endpoint.
        pub frontend_port: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the idle timeout in minutes for TCP connections.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the load balancing distribution type used by the Load Balancer.
        pub load_distribution: pulumi_gestalt_rust::Output<String>,
        pub loadbalancer_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A reference to a Probe used by this Load Balancing Rule.
        pub probe_id: pulumi_gestalt_rust::Output<String>,
        /// The transport protocol for the external endpoint.
        pub protocol: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLbRuleArgs,
    ) -> GetLbRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let loadbalancer_id_binding_1 = args.loadbalancer_id.get_output(context);
        let loadbalancer_id_binding = loadbalancer_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:lb/getLBRule:getLBRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "loadbalancerId".into(),
                    value: &loadbalancer_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLbRuleResult {
            backend_address_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendAddressPoolId"),
            ),
            backend_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendPort"),
            ),
            disable_outbound_snat: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableOutboundSnat"),
            ),
            enable_floating_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableFloatingIp"),
            ),
            enable_tcp_reset: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableTcpReset"),
            ),
            frontend_ip_configuration_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendIpConfigurationName"),
            ),
            frontend_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendPort"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            idle_timeout_in_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("idleTimeoutInMinutes"),
            ),
            load_distribution: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadDistribution"),
            ),
            loadbalancer_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadbalancerId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            probe_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("probeId"),
            ),
            protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
        }
    }
}
