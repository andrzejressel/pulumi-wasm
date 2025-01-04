pub mod get_trigger_schedules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTriggerSchedulesArgs {
        /// The ID of the Azure Data Factory to fetch trigger schedules from.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetTriggerSchedulesArgs) -> GetTriggerSchedulesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datafactory/getTriggerSchedules:getTriggerSchedules".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "items".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTriggerSchedulesResult {
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            items: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("items").unwrap(),
            ),
        }
    }
}
