/// Identical to the `random.RandomString` resource with the exception that the
/// result is treated as sensitive and, thus, _not_ displayed in console output.
///
/// > **Note:** All attributes including the generated password will be stored in
/// the raw state as plain-text.
///
/// This resource *does* use a cryptographic random number generator.
///
///
/// ## Example Usage
///
///
/// ## Import
///
/// Random Password can be imported by specifying the value of the string
///
/// ```sh
///  $ pulumi import random:index/randomPassword:RandomPassword password securepassword
/// ```
///
///
pub mod random_password {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomPasswordArgs {
        #[builder(into, default)]
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into)]
        pub length: pulumi_wasm_rust::Output<i32>,
        #[builder(into, default)]
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        #[builder(into, default)]
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RandomPasswordResult {
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub length: pulumi_wasm_rust::Output<i32>,
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        pub result: pulumi_wasm_rust::Output<String>,
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RandomPasswordArgs) -> RandomPasswordResult {
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
            type_: "random:index/randomPassword:RandomPassword".into(),
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
        RandomPasswordResult {
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
