pub mod get_db_system_shapes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDbSystemShapesArgs {
        /// The Azure Region to query for the system shapes in.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDbSystemShapesResult {
        /// A `db_system_shapes` block as defined below.
        pub db_system_shapes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::oracle::GetDbSystemShapesDbSystemShape>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDbSystemShapesArgs,
    ) -> GetDbSystemShapesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getDbSystemShapes:getDbSystemShapes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDbSystemShapesResult {
            db_system_shapes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dbSystemShapes"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
        }
    }
}
