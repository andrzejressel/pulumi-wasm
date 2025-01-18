/// The resource `random.RandomShuffle` generates a random permutation of a list of strings given as an argument.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   az:
///     type: random:RandomShuffle
///     properties:
///       inputs:
///         - us-west-1a
///         - us-west-1c
///         - us-west-1d
///         - us-west-1e
///       resultCount: 2
///   example:
///     type: aws:elb:LoadBalancer
///     properties:
///       # Place the ELB in any two of the given availability zones, selected
///       #   # at random.
///       availabilityZones: ${az.results}
/// ```
pub mod random_shuffle {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomShuffleArgs {
        /// The list of strings to shuffle.
        #[builder(into)]
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
        #[builder(into, default)]
        pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
        #[builder(into, default)]
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RandomShuffleResult {
        /// The list of strings to shuffle.
        pub inputs: pulumi_wasm_rust::Output<Vec<String>>,
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
        pub result_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Random permutation of the list of strings given in `input`.
        pub results: pulumi_wasm_rust::Output<Vec<String>>,
        /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RandomShuffleArgs) -> RandomShuffleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let inputs_binding = args.inputs.get_inner();
        let keepers_binding = args.keepers.get_inner();
        let result_count_binding = args.result_count.get_inner();
        let seed_binding = args.seed.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "random:index/randomShuffle:RandomShuffle".into(),
            name: name.to_string(),
            version: super::get_version(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "inputs".into(),
                },
                register_interface::ResultField {
                    name: "keepers".into(),
                },
                register_interface::ResultField {
                    name: "resultCount".into(),
                },
                register_interface::ResultField {
                    name: "results".into(),
                },
                register_interface::ResultField {
                    name: "seed".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RandomShuffleResult {
            inputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputs").unwrap(),
            ),
            keepers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepers").unwrap(),
            ),
            result_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resultCount").unwrap(),
            ),
            results: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("results").unwrap(),
            ),
            seed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("seed").unwrap(),
            ),
        }
    }
}
