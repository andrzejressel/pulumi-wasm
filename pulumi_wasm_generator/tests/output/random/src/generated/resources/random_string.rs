/// The resource `random.RandomString` generates a random permutation of alphanumeric
/// characters and optionally special characters.
///
/// This resource *does* use a cryptographic random number generator.
///
/// Historically this resource's intended usage has been ambiguous as the original example
/// used it in a password. For backwards compatibility it will
/// continue to exist. For unique ids please use the `random.RandomId` resource, for sensitive
/// random values please use the `random.RandomPassword` resource.
///
///
/// ## Example Usage
///
///
/// ## Import
///
/// Strings can be imported by just specifying the value of the string
///
/// ```sh
///  $ pulumi import random:index/randomString:RandomString test test
/// ```
///
///
pub mod random_string {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomStringArgs {
        /// Arbitrary map of values that, when changed, will
        /// trigger a new id to be generated.
        ///
        #[builder(into, default)]
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length of the string desired
        ///
        #[builder(into)]
        pub length: pulumi_wasm_rust::Output<i32>,
        /// (default true) Include lowercase alphabet characters
        /// in random string.
        ///
        #[builder(into, default)]
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        /// (default 0) Minimum number of lowercase alphabet
        /// characters in random string.
        ///
        #[builder(into, default)]
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default 0) Minimum number of numeric characters
        /// in random string.
        ///
        #[builder(into, default)]
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default 0) Minimum number of special characters
        /// in random string.
        ///
        #[builder(into, default)]
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default 0) Minimum number of uppercase alphabet
        /// characters in random string.
        ///
        #[builder(into, default)]
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default true) Include numeric characters in random
        /// string.
        ///
        #[builder(into, default)]
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        /// Supply your own list of special characters to
        /// use for string generation.  This overrides the default character list in the special
        /// argument.  The special argument must still be set to true for any overwritten
        /// characters to be used in generation.
        ///
        #[builder(into, default)]
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        /// (default true) Include special characters in random
        /// string. These are `!@#$%&*()-_=+[]{}<>:?`
        ///
        #[builder(into, default)]
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        /// (default true) Include uppercase alphabet characters
        /// in random string.
        ///
        #[builder(into, default)]
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RandomStringResult {
        /// Arbitrary map of values that, when changed, will
        /// trigger a new id to be generated.
        ///
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length of the string desired
        ///
        pub length: pulumi_wasm_rust::Output<i32>,
        /// (default true) Include lowercase alphabet characters
        /// in random string.
        ///
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        /// (default 0) Minimum number of lowercase alphabet
        /// characters in random string.
        ///
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default 0) Minimum number of numeric characters
        /// in random string.
        ///
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default 0) Minimum number of special characters
        /// in random string.
        ///
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default 0) Minimum number of uppercase alphabet
        /// characters in random string.
        ///
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        /// (default true) Include numeric characters in random
        /// string.
        ///
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        /// Supply your own list of special characters to
        /// use for string generation.  This overrides the default character list in the special
        /// argument.  The special argument must still be set to true for any overwritten
        /// characters to be used in generation.
        ///
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        /// Random string generated.
        ///
        pub result: pulumi_wasm_rust::Output<String>,
        /// (default true) Include special characters in random
        /// string. These are `!@#$%&*()-_=+[]{}<>:?`
        ///
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        /// (default true) Include uppercase alphabet characters
        /// in random string.
        ///
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RandomStringArgs) -> RandomStringResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let keepers_binding = args.keepers.get_inner();
        let length_binding = args.length.get_inner();
        let lower_binding = args.lower.get_inner();
        let min_lower_binding = args.min_lower.get_inner();
        let min_numeric_binding = args.min_numeric.get_inner();
        let min_special_binding = args.min_special.get_inner();
        let min_upper_binding = args.min_upper.get_inner();
        let number_binding = args.number.get_inner();
        let override_special_binding = args.override_special.get_inner();
        let special_binding = args.special.get_inner();
        let upper_binding = args.upper.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "random:index/randomString:RandomString".into(),
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
                    name: "lower".into(),
                    value: &lower_binding,
                },
                register_interface::ObjectField {
                    name: "minLower".into(),
                    value: &min_lower_binding,
                },
                register_interface::ObjectField {
                    name: "minNumeric".into(),
                    value: &min_numeric_binding,
                },
                register_interface::ObjectField {
                    name: "minSpecial".into(),
                    value: &min_special_binding,
                },
                register_interface::ObjectField {
                    name: "minUpper".into(),
                    value: &min_upper_binding,
                },
                register_interface::ObjectField {
                    name: "number".into(),
                    value: &number_binding,
                },
                register_interface::ObjectField {
                    name: "overrideSpecial".into(),
                    value: &override_special_binding,
                },
                register_interface::ObjectField {
                    name: "special".into(),
                    value: &special_binding,
                },
                register_interface::ObjectField {
                    name: "upper".into(),
                    value: &upper_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "keepers".into(),
                },
                register_interface::ResultField {
                    name: "length".into(),
                },
                register_interface::ResultField {
                    name: "lower".into(),
                },
                register_interface::ResultField {
                    name: "minLower".into(),
                },
                register_interface::ResultField {
                    name: "minNumeric".into(),
                },
                register_interface::ResultField {
                    name: "minSpecial".into(),
                },
                register_interface::ResultField {
                    name: "minUpper".into(),
                },
                register_interface::ResultField {
                    name: "number".into(),
                },
                register_interface::ResultField {
                    name: "overrideSpecial".into(),
                },
                register_interface::ResultField {
                    name: "result".into(),
                },
                register_interface::ResultField {
                    name: "special".into(),
                },
                register_interface::ResultField {
                    name: "upper".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RandomStringResult {
            keepers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepers").unwrap(),
            ),
            length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("length").unwrap(),
            ),
            lower: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lower").unwrap(),
            ),
            min_lower: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minLower").unwrap(),
            ),
            min_numeric: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minNumeric").unwrap(),
            ),
            min_special: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minSpecial").unwrap(),
            ),
            min_upper: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minUpper").unwrap(),
            ),
            number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("number").unwrap(),
            ),
            override_special: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overrideSpecial").unwrap(),
            ),
            result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("result").unwrap(),
            ),
            special: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("special").unwrap(),
            ),
            upper: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upper").unwrap(),
            ),
        }
    }
}
