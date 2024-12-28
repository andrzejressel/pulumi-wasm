/// The resource `random.RandomId` generates random numbers that are intended to be
/// used as unique identifiers for other resources.
///
/// This resource *does* use a cryptographic random number generator in order
/// to minimize the chance of collisions, making the results of this resource
/// when a 16-byte identifier is requested of equivalent uniqueness to a
/// type-4 UUID.
///
/// This resource can be used in conjunction with resources that have
/// the `create_before_destroy` lifecycle flag set to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
///
/// ## Example Usage
///
/// The following example shows how to generate a unique name for an AWS EC2
/// instance that changes each time a new AMI id is selected.
///
///
/// ## Import
///
/// Random Ids can be imported using the `b64_url` with an optional `prefix`. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example with no prefix
///
/// ```sh
///  $ pulumi import random:index/randomId:RandomId server p-9hUg
/// ```
///
///  Example with prefix (prefix is separated by a `,`)
///
/// ```sh
///  $ pulumi import random:index/randomId:RandomId server my-prefix-,p-9hUg
/// ```
///
///
pub mod random_id {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomIdArgs {
        /// The number of random bytes to produce. The
        /// minimum value is 1, which produces eight bits of randomness.
        ///
        #[builder(into)]
        pub byte_length: pulumi_wasm_rust::Output<i32>,
        /// Arbitrary map of values that, when changed, will
        /// trigger a new id to be generated.
        ///
        #[builder(into, default)]
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Arbitrary string to prefix the output value with. This
        /// string is supplied as-is, meaning it is not guaranteed to be URL-safe or
        /// base64 encoded.
        ///
        #[builder(into, default)]
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RandomIdResult {
        pub b64: pulumi_wasm_rust::Output<String>,
        /// The generated id presented in base64 without additional transformations.
        ///
        pub b64_std: pulumi_wasm_rust::Output<String>,
        /// The generated id presented in base64, using the URL-friendly character set: case-sensitive letters, digits and the characters `_` and `-`.
        ///
        pub b64_url: pulumi_wasm_rust::Output<String>,
        /// The number of random bytes to produce. The
        /// minimum value is 1, which produces eight bits of randomness.
        ///
        pub byte_length: pulumi_wasm_rust::Output<i32>,
        /// The generated id presented in non-padded decimal digits.
        ///
        pub dec: pulumi_wasm_rust::Output<String>,
        /// The generated id presented in padded hexadecimal digits. This result will always be twice as long as the requested byte length.
        ///
        pub hex: pulumi_wasm_rust::Output<String>,
        /// Arbitrary map of values that, when changed, will
        /// trigger a new id to be generated.
        ///
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Arbitrary string to prefix the output value with. This
        /// string is supplied as-is, meaning it is not guaranteed to be URL-safe or
        /// base64 encoded.
        ///
        pub prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RandomIdArgs) -> RandomIdResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let byte_length_binding = args.byte_length.get_inner();
        let keepers_binding = args.keepers.get_inner();
        let prefix_binding = args.prefix.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "random:index/randomId:RandomId".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "byteLength".into(),
                    value: &byte_length_binding,
                },
                register_interface::ObjectField {
                    name: "keepers".into(),
                    value: &keepers_binding,
                },
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "b64".into(),
                },
                register_interface::ResultField {
                    name: "b64Std".into(),
                },
                register_interface::ResultField {
                    name: "b64Url".into(),
                },
                register_interface::ResultField {
                    name: "byteLength".into(),
                },
                register_interface::ResultField {
                    name: "dec".into(),
                },
                register_interface::ResultField {
                    name: "hex".into(),
                },
                register_interface::ResultField {
                    name: "keepers".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RandomIdResult {
            b64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("b64").unwrap(),
            ),
            b64_std: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("b64Std").unwrap(),
            ),
            b64_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("b64Url").unwrap(),
            ),
            byte_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("byteLength").unwrap(),
            ),
            dec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dec").unwrap(),
            ),
            hex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hex").unwrap(),
            ),
            keepers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepers").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
        }
    }
}
