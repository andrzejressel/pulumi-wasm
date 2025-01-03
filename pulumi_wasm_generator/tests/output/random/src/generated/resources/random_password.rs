/// Identical to random_string.
///
/// This resource *does* use a cryptographic random number generator.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   password:
///     type: random:RandomPassword
///     properties:
///       length: 16
///       special: true
///       overrideSpecial: '!#$%&*()-_=+[]{}<>:?'
///   example:
///     type: aws:rds:Instance
///     properties:
///       instanceClass: db.t3.micro
///       allocatedStorage: 64
///       engine: mysql
///       username: someone
///       password: ${password.result}
/// ```
///
/// ## Import
///
/// You can import external passwords into your Pulumi programs as follows:
///
/// ```sh
///  $ import random:index/randomPassword:RandomPassword newPassword supersecret
/// ```
///
/// This command will encode the `supersecret` token in Pulumi state and generate a code suggestion to include a new RandomPassword resource in your Pulumi program. Include the suggested code and do a `pulumi up`. Your secret password is now securely stored in Pulumi, and you can reference it in your Pulumi program as `newPassword.result`.
pub mod random_password {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomPasswordArgs {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`).
        #[builder(into)]
        pub length: pulumi_wasm_rust::Output<i32>,
        /// Include lowercase alphabet characters in the result. Default value is `true`.
        #[builder(into, default)]
        pub lower: pulumi_wasm_rust::Output<Option<bool>>,
        /// Minimum number of lowercase alphabet characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_lower: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum number of numeric characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_numeric: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum number of special characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_special: pulumi_wasm_rust::Output<Option<i32>>,
        /// Minimum number of uppercase alphabet characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_upper: pulumi_wasm_rust::Output<Option<i32>>,
        /// Include numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead.
        #[builder(into, default)]
        pub number: pulumi_wasm_rust::Output<Option<bool>>,
        /// Include numeric characters in the result. Default value is `true`.
        #[builder(into, default)]
        pub numeric: pulumi_wasm_rust::Output<Option<bool>>,
        /// Supply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation.
        #[builder(into, default)]
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        /// Include special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`.
        #[builder(into, default)]
        pub special: pulumi_wasm_rust::Output<Option<bool>>,
        /// Include uppercase alphabet characters in the result. Default value is `true`.
        #[builder(into, default)]
        pub upper: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RandomPasswordResult {
        /// A bcrypt hash of the generated random string. **NOTE**: If the generated random string is greater than 72 bytes in length, `bcrypt_hash` will contain a hash of the first 72 bytes.
        pub bcrypt_hash: pulumi_wasm_rust::Output<String>,
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`).
        pub length: pulumi_wasm_rust::Output<i32>,
        /// Include lowercase alphabet characters in the result. Default value is `true`.
        pub lower: pulumi_wasm_rust::Output<bool>,
        /// Minimum number of lowercase alphabet characters in the result. Default value is `0`.
        pub min_lower: pulumi_wasm_rust::Output<i32>,
        /// Minimum number of numeric characters in the result. Default value is `0`.
        pub min_numeric: pulumi_wasm_rust::Output<i32>,
        /// Minimum number of special characters in the result. Default value is `0`.
        pub min_special: pulumi_wasm_rust::Output<i32>,
        /// Minimum number of uppercase alphabet characters in the result. Default value is `0`.
        pub min_upper: pulumi_wasm_rust::Output<i32>,
        /// Include numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead.
        pub number: pulumi_wasm_rust::Output<bool>,
        /// Include numeric characters in the result. Default value is `true`.
        pub numeric: pulumi_wasm_rust::Output<bool>,
        /// Supply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation.
        pub override_special: pulumi_wasm_rust::Output<Option<String>>,
        /// The generated random string.
        pub result: pulumi_wasm_rust::Output<String>,
        /// Include special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`.
        pub special: pulumi_wasm_rust::Output<bool>,
        /// Include uppercase alphabet characters in the result. Default value is `true`.
        pub upper: pulumi_wasm_rust::Output<bool>,
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
        let numeric_binding = args.numeric.get_inner();
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
                    name: "numeric".into(),
                    value: &numeric_binding,
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
                    name: "bcryptHash".into(),
                },
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
                    name: "numeric".into(),
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
            bcrypt_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bcryptHash").unwrap(),
            ),
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
            numeric: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numeric").unwrap(),
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
