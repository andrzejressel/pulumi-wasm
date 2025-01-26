pub mod get_hosted_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHostedZoneArgs {
        /// Region you'd like the zone for. By default, fetches the current region.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetHostedZoneResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Region of the hosted zone.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetHostedZoneArgs,
    ) -> GetHostedZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticbeanstalk/getHostedZone:getHostedZone".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetHostedZoneResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
        }
    }
}
