#[allow(clippy::doc_lazy_continuation)]
pub mod get_lb_outbound_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLbOutboundRuleArgs {
        /// The ID of the Load Balancer in which the Outbound Rule exists.
        #[builder(into)]
        pub loadbalancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Load Balancer Outbound Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLbOutboundRuleResult {
        /// The number of outbound ports used for NAT.
        pub allocated_outbound_ports: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the Backend Address Pool. Outbound traffic is randomly load balanced across IPs in the backend IPs.
        pub backend_address_pool_id: pulumi_gestalt_rust::Output<String>,
        /// A `frontend_ip_configuration` block as defined below.
        pub frontend_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lb::GetLbOutboundRuleFrontendIpConfiguration>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The timeout for the TCP idle connection.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<i32>,
        pub loadbalancer_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Frontend IP Configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The transport protocol for the external endpoint.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// Is the bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination enabled? This value is useful when the protocol is set to TCP.
        pub tcp_reset_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLbOutboundRuleArgs,
    ) -> GetLbOutboundRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let loadbalancer_id_binding = args
            .loadbalancer_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:lb/getLBOutboundRule:getLBOutboundRule".into(),
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
        GetLbOutboundRuleResult {
            allocated_outbound_ports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocatedOutboundPorts"),
            ),
            backend_address_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backendAddressPoolId"),
            ),
            frontend_ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendIpConfigurations"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            idle_timeout_in_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("idleTimeoutInMinutes"),
            ),
            loadbalancer_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadbalancerId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            tcp_reset_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tcpResetEnabled"),
            ),
        }
    }
}
