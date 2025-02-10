#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_backend_address_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackendAddressPoolArgs {
        /// The ID of the Load Balancer in which the Backend Address Pool exists.
        #[builder(into)]
        pub loadbalancer_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Backend Address Pool.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBackendAddressPoolResult {
        /// A list of `backend_address` block as defined below.
        pub backend_addresses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lb::GetBackendAddressPoolBackendAddress>,
        >,
        /// A list of references to IP addresses defined in network interfaces.
        pub backend_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::lb::GetBackendAddressPoolBackendIpConfiguration,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of the Load Balancing Inbound NAT Rules associated with this Backend Address Pool.
        pub inbound_nat_rules: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of the Load Balancing Rules associated with this Backend Address Pool.
        pub load_balancing_rules: pulumi_gestalt_rust::Output<Vec<String>>,
        pub loadbalancer_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Backend Address.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of the Load Balancing Outbound Rules associated with this Backend Address Pool.
        pub outbound_rules: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBackendAddressPoolArgs,
    ) -> GetBackendAddressPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let loadbalancer_id_binding = args.loadbalancer_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:lb/getBackendAddressPool:getBackendAddressPool".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadbalancerId".into(),
                    value: loadbalancer_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBackendAddressPoolResult {
            backend_addresses: o.get_field("backendAddresses"),
            backend_ip_configurations: o.get_field("backendIpConfigurations"),
            id: o.get_field("id"),
            inbound_nat_rules: o.get_field("inboundNatRules"),
            load_balancing_rules: o.get_field("loadBalancingRules"),
            loadbalancer_id: o.get_field("loadbalancerId"),
            name: o.get_field("name"),
            outbound_rules: o.get_field("outboundRules"),
        }
    }
}
