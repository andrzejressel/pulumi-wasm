/// Access Keys Configuration defines the rotation policy for the keys
/// that access will use to sign data.
pub mod access_keys_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessKeysConfigurationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Number of days to trigger a rotation of the keys.
        #[builder(into, default)]
        pub key_rotation_interval_days: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct AccessKeysConfigurationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Number of days to trigger a rotation of the keys.
        pub key_rotation_interval_days: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccessKeysConfigurationArgs,
    ) -> AccessKeysConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let key_rotation_interval_days_binding = args
            .key_rotation_interval_days
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accessKeysConfiguration:AccessKeysConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyRotationIntervalDays".into(),
                    value: &key_rotation_interval_days_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "keyRotationIntervalDays".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessKeysConfigurationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            key_rotation_interval_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyRotationIntervalDays").unwrap(),
            ),
        }
    }
}