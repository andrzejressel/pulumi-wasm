pub mod get_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlanArgs {
        /// The Amazon Resource Name (ARN) of the contact or escalation plan.
        #[builder(into)]
        pub contact_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPlanResult {
        pub contact_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of stages. A contact has an engagement plan with stages that contact specified contact channels. An escalation plan uses stages that contact specified contacts.
        pub stages: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssmcontacts::GetPlanStage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPlanArgs) -> GetPlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_id_binding = args.contact_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssmcontacts/getPlan:getPlan".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactId".into(),
                    value: &contact_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contactId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "stages".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPlanResult {
            contact_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            stages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stages").unwrap(),
            ),
        }
    }
}
