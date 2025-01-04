/// Resource for managing an AWS Payment Cryptography Control Plane Key.
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
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Payment Cryptography Control Plane Key using the `arn:aws:payment-cryptography:us-east-1:123456789012:key/qtbojf64yshyvyzf`. For example:
///
/// ```sh
/// $ pulumi import aws:paymentcryptography/key:Key example arn:aws:payment-cryptography:us-east-1:123456789012:key/qtbojf64yshyvyzf
/// ```
pub mod key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyArgs {
        #[builder(into, default)]
        pub deletion_window_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Whether to enable the key.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the key is exportable from the service.
        #[builder(into)]
        pub exportable: pulumi_wasm_rust::Output<bool>,
        /// Role of the key, the algorithm it supports, and the cryptographic operations allowed with the key.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub key_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::paymentcryptography::KeyKeyAttributes>,
        >,
        /// Algorithm that AWS Payment Cryptography uses to calculate the key check value (KCV).
        #[builder(into, default)]
        pub key_check_value_algorithm: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::paymentcryptography::KeyTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct KeyResult {
        /// ARN of the key.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub deletion_window_in_days: pulumi_wasm_rust::Output<i32>,
        /// Whether to enable the key.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Whether the key is exportable from the service.
        pub exportable: pulumi_wasm_rust::Output<bool>,
        /// Role of the key, the algorithm it supports, and the cryptographic operations allowed with the key.
        ///
        /// The following arguments are optional:
        pub key_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::paymentcryptography::KeyKeyAttributes>,
        >,
        /// Key check value (KCV) is used to check if all parties holding a given key have the same key or to detect that a key has changed.
        pub key_check_value: pulumi_wasm_rust::Output<String>,
        /// Algorithm that AWS Payment Cryptography uses to calculate the key check value (KCV).
        pub key_check_value_algorithm: pulumi_wasm_rust::Output<String>,
        /// Source of the key material.
        pub key_origin: pulumi_wasm_rust::Output<String>,
        /// State of key that is being created or deleted.
        pub key_state: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the WorkSpaces Connection Alias. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::paymentcryptography::KeyTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KeyArgs) -> KeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deletion_window_in_days_binding = args.deletion_window_in_days.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let exportable_binding = args.exportable.get_inner();
        let key_attributes_binding = args.key_attributes.get_inner();
        let key_check_value_algorithm_binding = args
            .key_check_value_algorithm
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:paymentcryptography/key:Key".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionWindowInDays".into(),
                    value: &deletion_window_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "exportable".into(),
                    value: &exportable_binding,
                },
                register_interface::ObjectField {
                    name: "keyAttributes".into(),
                    value: &key_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "keyCheckValueAlgorithm".into(),
                    value: &key_check_value_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "deletionWindowInDays".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "exportable".into(),
                },
                register_interface::ResultField {
                    name: "keyAttributes".into(),
                },
                register_interface::ResultField {
                    name: "keyCheckValue".into(),
                },
                register_interface::ResultField {
                    name: "keyCheckValueAlgorithm".into(),
                },
                register_interface::ResultField {
                    name: "keyOrigin".into(),
                },
                register_interface::ResultField {
                    name: "keyState".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            deletion_window_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionWindowInDays").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            exportable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportable").unwrap(),
            ),
            key_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyAttributes").unwrap(),
            ),
            key_check_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyCheckValue").unwrap(),
            ),
            key_check_value_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyCheckValueAlgorithm").unwrap(),
            ),
            key_origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyOrigin").unwrap(),
            ),
            key_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyState").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
