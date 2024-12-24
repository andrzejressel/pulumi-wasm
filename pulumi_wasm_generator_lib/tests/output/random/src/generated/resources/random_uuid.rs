#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomUuidArgs {
    /// Arbitrary map of values that, when changed, will
    /// trigger a new uuid to be generated.
    ///
    #[builder(into, default)]
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
}
pub struct RandomUuidResult {
    /// Arbitrary map of values that, when changed, will
    /// trigger a new uuid to be generated.
    ///
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// The generated uuid presented in string format.
    ///
    pub result: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RandomUuidArgs) -> RandomUuidResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let keepers_binding = args.keepers.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "random:index/randomUuid:RandomUuid".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "keepers".into(),
                value: &keepers_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "keepers".into() },
            register_interface::ResultField { name : "result".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    RandomUuidResult {
        keepers: into_domain(hashmap.remove("keepers").unwrap()),
        result: into_domain(hashmap.remove("result").unwrap()),
    }
}
