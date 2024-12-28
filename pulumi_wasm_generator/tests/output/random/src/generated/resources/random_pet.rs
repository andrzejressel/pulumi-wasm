/// The resource `random.RandomPet` generates random pet names that are intended to be
/// used as unique identifiers for other resources.
///
/// This resource can be used in conjunction with resources that have
/// the `create_before_destroy` lifecycle flag set, to avoid conflicts with
/// unique names during the brief period where both the old and new resources
/// exist concurrently.
///
///
/// ## Example Usage
///
/// The following example shows how to generate a unique pet name for an AWS EC2
/// instance that changes each time a new AMI id is selected.
///
///
/// The result of the above will set the Name of the AWS Instance to
/// `web-server-simple-snake`.
pub mod random_pet {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RandomPetArgs) -> RandomPetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "keepers".into(),
                },
                register_interface::ResultField {
                    name: "length".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
                register_interface::ResultField {
                    name: "separator".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RandomPetResult {
            keepers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepers").unwrap(),
            ),
            length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("length").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
            separator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("separator").unwrap(),
            ),
        }
    }
}
