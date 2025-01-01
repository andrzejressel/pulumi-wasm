/// The resource `random.RandomUuid` generates a random uuid string that is intended to be used as a unique identifier for other resources.
///
/// This resource uses [hashicorp/go-uuid](https://github.com/hashicorp/go-uuid) to generate a UUID-formatted string for use with services needing a unique string identifier.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testRandomUuid:
///     type: random:RandomUuid
///   testResourceGroup:
///     type: azure:core:ResourceGroup
///     properties:
///       location: Central US
/// ```
///
/// ## Import
///
/// Random UUID's can be imported. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs.
///
/// ```sh
///  $ pulumi import random:index/randomUuid:RandomUuid main aabbccdd-eeff-0011-2233-445566778899
/// ```
///
///
pub mod random_uuid {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomUuidArgs {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RandomUuidResult {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The generated uuid presented in string format.
        pub result: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RandomUuidArgs) -> RandomUuidResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let keepers_binding = args.keepers.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "random:index/randomUuid:RandomUuid".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keepers".into(),
                    value: &keepers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "keepers".into(),
                },
                register_interface::ResultField {
                    name: "result".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RandomUuidResult {
            keepers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepers").unwrap(),
            ),
            result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("result").unwrap(),
            ),
        }
    }
}
