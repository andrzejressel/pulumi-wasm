#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RandomPetArgs {
    /// Arbitrary map of values that, when changed, will
    /// trigger a new id to be generated.
    ///
    #[builder(into, default)]
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// The length (in words) of the pet name.
    ///
    #[builder(into, default)]
    pub length: pulumi_wasm_rust::Output<Option<i32>>,
    /// A string to prefix the name with.
    ///
    #[builder(into, default)]
    pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    /// The character to separate words in the pet name.
    ///
    #[builder(into, default)]
    pub separator: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct RandomPetResult {
    /// Arbitrary map of values that, when changed, will
    /// trigger a new id to be generated.
    ///
    pub keepers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
    /// The length (in words) of the pet name.
    ///
    pub length: pulumi_wasm_rust::Output<Option<i32>>,
    /// A string to prefix the name with.
    ///
    pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    /// The character to separate words in the pet name.
    ///
    pub separator: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: RandomPetArgs) -> RandomPetResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let keepers_binding = args.keepers.get_inner();
    let length_binding = args.length.get_inner();
    let prefix_binding = args.prefix.get_inner();
    let separator_binding = args.separator.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "random:index/randomPet:RandomPet".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "keepers".into(),
                value: &keepers_binding,
            },
            register_interface::ObjectField {
                name: "length".into(),
                value: &length_binding,
            },
            register_interface::ObjectField {
                name: "prefix".into(),
                value: &prefix_binding,
            },
            register_interface::ObjectField {
                name: "separator".into(),
                value: &separator_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "keepers".into() },
            register_interface::ResultField { name : "length".into() },
            register_interface::ResultField { name : "prefix".into() },
            register_interface::ResultField { name : "separator".into() },
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
    RandomPetResult {
        keepers: into_domain(hashmap.remove("keepers").unwrap()),
        length: into_domain(hashmap.remove("length").unwrap()),
        prefix: into_domain(hashmap.remove("prefix").unwrap()),
        separator: into_domain(hashmap.remove("separator").unwrap()),
    }
}
