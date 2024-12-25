#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomIntegerArgs {
    /// Arbitrary map of values that, when changed, will
    /// trigger a new id to be generated.
    ///
    #[builder(into, default)]
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// The maximum inclusive value of the range.
    ///
    #[builder(into)]
    pub max: pulumi_wasm_rust::Output<i32>,
    /// The minimum inclusive value of the range.
    ///
    #[builder(into)]
    pub min: pulumi_wasm_rust::Output<i32>,
    /// A custom seed to always produce the same value.
    ///
    #[builder(into, default)]
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct RandomIntegerResult {
    /// Arbitrary map of values that, when changed, will
    /// trigger a new id to be generated.
    ///
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// The maximum inclusive value of the range.
    ///
    pub max: pulumi_wasm_rust::Output<i32>,
    /// The minimum inclusive value of the range.
    ///
    pub min: pulumi_wasm_rust::Output<i32>,
    /// (int) The random Integer result.
    ///
    pub result: pulumi_wasm_rust::Output<i32>,
    /// A custom seed to always produce the same value.
    ///
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: RandomIntegerArgs) -> RandomIntegerResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let keepers_binding = args.keepers.get_inner();
    let max_binding = args.max.get_inner();
    let min_binding = args.min.get_inner();
    let seed_binding = args.seed.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "random:index/randomInteger:RandomInteger".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "keepers".into(),
                value: &keepers_binding,
            },
            register_interface::ObjectField {
                name: "max".into(),
                value: &max_binding,
            },
            register_interface::ObjectField {
                name: "min".into(),
                value: &min_binding,
            },
            register_interface::ObjectField {
                name: "seed".into(),
                value: &seed_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "keepers".into() },
            register_interface::ResultField { name : "max".into() },
            register_interface::ResultField { name : "min".into() },
            register_interface::ResultField { name : "result".into() },
            register_interface::ResultField { name : "seed".into() },
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
    RandomIntegerResult {
        keepers: into_domain(hashmap.remove("keepers").unwrap()),
        max: into_domain(hashmap.remove("max").unwrap()),
        min: into_domain(hashmap.remove("min").unwrap()),
        result: into_domain(hashmap.remove("result").unwrap()),
        seed: into_domain(hashmap.remove("seed").unwrap()),
    }
}
