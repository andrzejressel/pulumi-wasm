#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_express_route_circuit {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExpressRouteCircuitArgs {
        /// The name of the ExpressRoute circuit.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the ExpressRoute circuit exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExpressRouteCircuitResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the ExpressRoute circuit exists
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `peerings` block for the ExpressRoute circuit as documented below
        pub peerings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetExpressRouteCircuitPeering>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The string needed by the service provider to provision the ExpressRoute circuit.
        pub service_key: pulumi_gestalt_rust::Output<String>,
        /// A `service_provider_properties` block for the ExpressRoute circuit as documented below
        pub service_provider_properties: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetExpressRouteCircuitServiceProviderProperty,
            >,
        >,
        /// The ExpressRoute circuit provisioning state from your chosen service provider. Possible values are `NotProvisioned`, `Provisioning`, `Provisioned`, and `Deprovisioning`.
        pub service_provider_provisioning_state: pulumi_gestalt_rust::Output<String>,
        /// A `sku` block for the ExpressRoute circuit as documented below.
        pub sku: pulumi_gestalt_rust::Output<
            super::super::super::types::network::GetExpressRouteCircuitSku,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetExpressRouteCircuitArgs,
    ) -> GetExpressRouteCircuitResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getExpressRouteCircuit:getExpressRouteCircuit".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetExpressRouteCircuitResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            peerings: o.get_field("peerings"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_key: o.get_field("serviceKey"),
            service_provider_properties: o.get_field("serviceProviderProperties"),
            service_provider_provisioning_state: o
                .get_field("serviceProviderProvisioningState"),
            sku: o.get_field("sku"),
        }
    }
}
