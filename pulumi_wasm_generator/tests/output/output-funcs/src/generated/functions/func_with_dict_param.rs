#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FuncWithDictParamArgs {
    #[builder(into, default)]
    pub a: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    #[builder(into, default)]
    pub b: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct FuncWithDictParamResult {
    pub r: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: FuncWithDictParamArgs) -> FuncWithDictParamResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let a_binding = args.a.get_inner();
    let b_binding = args.b.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "mypkg::funcWithDictParam".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "a".into(),
                value: &a_binding,
            },
            register_interface::ObjectField {
                name: "b".into(),
                value: &b_binding,
            },
        ]),
        results: vec![register_interface::ResultField { name : "r".into() },],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    FuncWithDictParamResult {
        r: into_domain(hashmap.remove("r").unwrap()),
    }
}
