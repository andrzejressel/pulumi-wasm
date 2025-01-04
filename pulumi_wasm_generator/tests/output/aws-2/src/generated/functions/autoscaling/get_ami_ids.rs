pub mod get_ami_ids {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAmiIdsArgs {
        /// Filter used to scope the list e.g., by tags. See [related docs](http://docs.aws.amazon.com/AutoScaling/latest/APIReference/API_Filter.html).
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::autoscaling::GetAmiIdsFilter>>,
        >,
        /// List of autoscaling group names
        #[builder(into, default)]
        pub names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetAmiIdsResult {
        /// List of the Autoscaling Groups Arns in the current region.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::autoscaling::GetAmiIdsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of the Autoscaling Groups in the current region.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAmiIdsArgs) -> GetAmiIdsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let names_binding = args.names.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:autoscaling/getAmiIds:getAmiIds".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "names".into(),
                    value: &names_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arns".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "names".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAmiIdsResult {
            arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arns").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("names").unwrap(),
            ),
        }
    }
}
