/// The resource `random.RandomInteger` generates random values from a given range, described by the `min` and `max` attributes of a given resource.
///
/// This resource can be used in conjunction with resources that have
/// the `create_before_destroy` lifecycle flag set, to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
///
/// ## Example Usage
///
/// The following example shows how to generate a random priority between 1 and 50000 for
/// a `aws_alb_listener_rule` resource:
///
///
/// The result of the above will set a random priority.
///
/// ## Import
///
/// Random integers can be imported using the `result`, `min`, and `max`, with an optional `seed`. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example (values are separated by a `,`)
///
/// ```sh
///  $ pulumi import random:index/randomInteger:RandomInteger priority 15390,1,50000
/// ```
///
///
pub mod random_integer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RandomIntegerArgs) -> RandomIntegerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "keepers".into(),
                },
                register_interface::ResultField {
                    name: "max".into(),
                },
                register_interface::ResultField {
                    name: "min".into(),
                },
                register_interface::ResultField {
                    name: "result".into(),
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
        RandomIntegerResult {
            keepers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepers").unwrap(),
            ),
            max: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("max").unwrap(),
            ),
            min: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("min").unwrap(),
            ),
            result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("result").unwrap(),
            ),
            seed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("seed").unwrap(),
            ),
        }
    }
}
