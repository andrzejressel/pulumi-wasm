pub mod get_configuration_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationKeyArgs {
        /// Specifies the id of the App Configuration.
        #[builder(into)]
        pub configuration_store_id: pulumi_wasm_rust::Output<String>,
        /// The name of the App Configuration Key.
        #[builder(into)]
        pub key: pulumi_wasm_rust::Output<String>,
        /// The label of the App Configuration Key.
        #[builder(into, default)]
        pub label: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationKeyResult {
        pub configuration_store_id: pulumi_wasm_rust::Output<String>,
        /// The content type of the App Configuration Key.
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// The ETag of the key.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub key: pulumi_wasm_rust::Output<String>,
        pub label: pulumi_wasm_rust::Output<Option<String>>,
        /// Is this App Configuration Key be Locked to prevent changes.
        pub locked: pulumi_wasm_rust::Output<bool>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the App Configuration Key. It can either be `kv` (simple [key/value](https://docs.microsoft.com/azure/azure-app-configuration/concept-key-value)) or `vault` (where the value is a reference to a [Key Vault Secret](https://azure.microsoft.com/en-gb/services/key-vault/).
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The value of the App Configuration Key.
        pub value: pulumi_wasm_rust::Output<String>,
        /// The ID of the vault secret this App Configuration Key refers to, when `type` is `vault`.
        pub vault_key_reference: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConfigurationKeyArgs) -> GetConfigurationKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_store_id_binding = args.configuration_store_id.get_inner();
        let key_binding = args.key.get_inner();
        let label_binding = args.label.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appconfiguration/getConfigurationKey:getConfigurationKey"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationStoreId".into(),
                    value: &configuration_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "label".into(),
                    value: &label_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurationStoreId".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "label".into(),
                },
                register_interface::ResultField {
                    name: "locked".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
                register_interface::ResultField {
                    name: "vaultKeyReference".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConfigurationKeyResult {
            configuration_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationStoreId").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("label").unwrap(),
            ),
            locked: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locked").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
            vault_key_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vaultKeyReference").unwrap(),
            ),
        }
    }
}
