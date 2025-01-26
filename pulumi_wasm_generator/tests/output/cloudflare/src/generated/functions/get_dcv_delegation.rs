pub mod get_dcv_delegation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDcvDelegationArgs {
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDcvDelegationResult {
        /// The DCV Delegation hostname
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The DCV Delegation unique identifier
        pub id: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDcvDelegationArgs,
    ) -> GetDcvDelegationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getDcvDelegation:getDcvDelegation".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDcvDelegationResult {
            hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
