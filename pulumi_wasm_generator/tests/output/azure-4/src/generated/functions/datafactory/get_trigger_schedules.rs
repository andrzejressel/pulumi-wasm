pub mod get_trigger_schedules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTriggerSchedulesArgs {
        /// The ID of the Azure Data Factory to fetch trigger schedules from.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTriggerSchedulesResult {
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of trigger schedule names available in this Azure Data Factory.
        pub items: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTriggerSchedulesArgs,
    ) -> GetTriggerSchedulesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datafactory/getTriggerSchedules:getTriggerSchedules".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTriggerSchedulesResult {
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            items: pulumi_wasm_rust::__private::into_domain(o.extract_field("items")),
        }
    }
}
