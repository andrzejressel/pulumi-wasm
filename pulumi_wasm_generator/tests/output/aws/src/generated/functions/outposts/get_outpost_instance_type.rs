pub mod get_outpost_instance_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOutpostInstanceTypeArgs {
        /// Outpost ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Desired instance type. Conflicts with `preferred_instance_types`.
        #[builder(into, default)]
        pub instance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of preferred instance types. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. Conflicts with `instance_type`.
        #[builder(into, default)]
        pub preferred_instance_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetOutpostInstanceTypeResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_type: pulumi_wasm_rust::Output<String>,
        pub preferred_instance_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetOutpostInstanceTypeArgs) -> GetOutpostInstanceTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let preferred_instance_types_binding = args.preferred_instance_types.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:outposts/getOutpostInstanceType:getOutpostInstanceType".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "preferredInstanceTypes".into(),
                    value: &preferred_instance_types_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "preferredInstanceTypes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOutpostInstanceTypeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            preferred_instance_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredInstanceTypes").unwrap(),
            ),
        }
    }
}