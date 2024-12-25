#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomShuffleArgs {
    /// The list of strings to shuffle.
    ///
    #[builder(into)]
    pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
    /// Arbitrary map of values that, when changed, will
    /// trigger a new id to be generated.
    ///
    #[builder(into, default)]
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// The number of results to return. Defaults to
    /// the number of items in the `input` list. If fewer items are requested,
    /// some elements will be excluded from the result. If more items are requested,
    /// items will be repeated in the result but not more frequently than the number
    /// of items in the input list.
    ///
    #[builder(into, default)]
    pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Arbitrary string with which to seed the random number
    /// generator, in order to produce less-volatile permutations of the list.
    /// **Important:** Even with an identical seed, it is not guaranteed that the
    /// same permutation will be produced across different versions of the provider.
    /// This argument causes the result to be *less volatile*, but not fixed for
    /// all time.
    ///
    #[builder(into, default)]
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct RandomShuffleResult {
    /// The list of strings to shuffle.
    ///
    pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
    /// Arbitrary map of values that, when changed, will
    /// trigger a new id to be generated.
    ///
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// The number of results to return. Defaults to
    /// the number of items in the `input` list. If fewer items are requested,
    /// some elements will be excluded from the result. If more items are requested,
    /// items will be repeated in the result but not more frequently than the number
    /// of items in the input list.
    ///
    pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Random permutation of the list of strings given in `input`.
    ///
    pub results: pulumi_wasm_rust::Output<Vec<String>>,
    /// Arbitrary string with which to seed the random number
    /// generator, in order to produce less-volatile permutations of the list.
    /// **Important:** Even with an identical seed, it is not guaranteed that the
    /// same permutation will be produced across different versions of the provider.
    /// This argument causes the result to be *less volatile*, but not fixed for
    /// all time.
    ///
    pub seed: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: RandomShuffleArgs) -> RandomShuffleResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let inputs_binding = args.inputs.get_inner();
    let keepers_binding = args.keepers.get_inner();
    let result_count_binding = args.result_count.get_inner();
    let seed_binding = args.seed.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "random:index/randomShuffle:RandomShuffle".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "inputs".into(),
                value: &inputs_binding,
            },
            register_interface::ObjectField {
                name: "keepers".into(),
                value: &keepers_binding,
            },
            register_interface::ObjectField {
                name: "resultCount".into(),
                value: &result_count_binding,
            },
            register_interface::ObjectField {
                name: "seed".into(),
                value: &seed_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "inputs".into() },
            register_interface::ResultField { name : "keepers".into() },
            register_interface::ResultField { name : "resultCount".into() },
            register_interface::ResultField { name : "results".into() },
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
    RandomShuffleResult {
        inputs: into_domain(hashmap.remove("inputs").unwrap()),
        keepers: into_domain(hashmap.remove("keepers").unwrap()),
        result_count: into_domain(hashmap.remove("resultCount").unwrap()),
        results: into_domain(hashmap.remove("results").unwrap()),
        seed: into_domain(hashmap.remove("seed").unwrap()),
    }
}
