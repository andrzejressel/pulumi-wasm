pub mod get_resolver_firewall_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResolverFirewallConfigArgs {
        /// The ID of the VPC from Amazon VPC that the configuration is for.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResolverFirewallConfigResult {
        pub firewall_fail_open: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResolverFirewallConfigArgs,
    ) -> GetResolverFirewallConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getResolverFirewallConfig:getResolverFirewallConfig"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResolverFirewallConfigResult {
            firewall_fail_open: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallFailOpen"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
        }
    }
}
