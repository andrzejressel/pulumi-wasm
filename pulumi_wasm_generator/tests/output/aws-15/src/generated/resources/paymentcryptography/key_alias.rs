/// Resource for managing an AWS Payment Cryptography Control Plane Key Alias.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:paymentcryptography:Key
///     properties:
///       exportable: true
///       keyAttributes:
///         - keyAlgorithm: TDES_3KEY
///           keyClass: SYMMETRIC_KEY
///           keyUsage: TR31_P0_PIN_ENCRYPTION_KEY
///           keyModesOfUse:
///             - decrypt: true
///               encrypt: true
///               wrap: true
///               unwrap: true
///   testKeyAlias:
///     type: aws:paymentcryptography:KeyAlias
///     name: test
///     properties:
///       aliasName: alias/test-alias
///       keyArn: ${test.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Payment Cryptography Control Plane Key Alias using the `alias/4681482429376900170`. For example:
///
/// ```sh
/// $ pulumi import aws:paymentcryptography/keyAlias:KeyAlias example alias/4681482429376900170
/// ```
pub mod key_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyAliasArgs {
        /// Name of the Key Alias.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub alias_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the key.
        #[builder(into, default)]
        pub key_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyAliasResult {
        /// Name of the Key Alias.
        ///
        /// The following arguments are optional:
        pub alias_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the key.
        pub key_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KeyAliasArgs) -> KeyAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_name_binding = args.alias_name.get_inner();
        let key_arn_binding = args.key_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:paymentcryptography/keyAlias:KeyAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aliasName".into(),
                    value: &alias_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyArn".into(),
                    value: &key_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aliasName".into(),
                },
                register_interface::ResultField {
                    name: "keyArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeyAliasResult {
            alias_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliasName").unwrap(),
            ),
            key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyArn").unwrap(),
            ),
        }
    }
}
