pub mod get_activity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetActivityArgs {
        /// ARN that identifies the activity.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name that identifies the activity.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetActivityResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date the activity was created.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetActivityArgs) -> GetActivityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sfn/getActivity:getActivity".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetActivityResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}