pub mod get_custom_key_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCustomKeyStoreArgs {
        /// The ID for the custom key store.
        #[builder(into, default)]
        pub custom_key_store_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The user-specified friendly name for the custom key store.
        #[builder(into, default)]
        pub custom_key_store_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCustomKeyStoreResult {
        pub cloud_hsm_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the custom key store is connected to its CloudHSM cluster.
        pub connection_state: pulumi_wasm_rust::Output<String>,
        /// The date and time when the custom key store was created.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        pub custom_key_store_id: pulumi_wasm_rust::Output<String>,
        pub custom_key_store_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The trust anchor certificate of the associated CloudHSM cluster.
        pub trust_anchor_certificate: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCustomKeyStoreArgs,
    ) -> GetCustomKeyStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_key_store_id_binding = args
            .custom_key_store_id
            .get_output(context)
            .get_inner();
        let custom_key_store_name_binding = args
            .custom_key_store_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:kms/getCustomKeyStore:getCustomKeyStore".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customKeyStoreId".into(),
                    value: &custom_key_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "customKeyStoreName".into(),
                    value: &custom_key_store_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cloudHsmClusterId".into(),
                },
                register_interface::ResultField {
                    name: "connectionState".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "customKeyStoreId".into(),
                },
                register_interface::ResultField {
                    name: "customKeyStoreName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "trustAnchorCertificate".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCustomKeyStoreResult {
            cloud_hsm_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudHsmClusterId").unwrap(),
            ),
            connection_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionState").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            custom_key_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customKeyStoreId").unwrap(),
            ),
            custom_key_store_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customKeyStoreName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            trust_anchor_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustAnchorCertificate").unwrap(),
            ),
        }
    }
}
